pub mod process;
mod loader;
mod elf;

use BitmapAllocator;
use self::process::*;
use self::loader::*;
use memory::MemoryArea;
use core::slice;
use alloc::Vec;

static mut PROCESS_ALLOCATOR: BitmapAllocator = BitmapAllocator::new(0x0, 0x0, 0x0);
pub static mut PROCESS_LIST: Option<Vec<Process>> = None;

pub fn init(free_memory_areas: Vec<MemoryArea>) {
    // Set up an allocator for the process area
    if free_memory_areas.len() > 0 {
        let process_area = free_memory_areas[0];
        println!("Allocating processes from {:#x} to {:#x}.", process_area.base, process_area.base+process_area.size);
        unsafe {
            PROCESS_ALLOCATOR.set_bitmap_start(process_area.base);
            PROCESS_ALLOCATOR.set_block_size(process_area.size/100);
            PROCESS_ALLOCATOR.set_size(process_area.size);
            PROCESS_ALLOCATOR.init();
            PROCESS_LIST = Some(Vec::new());
        }
    } else {
        panic!("No space for process allocator.");
    }
}

pub fn add_process(process: Process) {
    unsafe {
        PROCESS_LIST.as_mut().unwrap().push(process);
    }
}

pub fn pop_process() -> Process {
    unsafe {
        PROCESS_LIST.as_mut().unwrap().pop().unwrap()
    }
}

pub fn get_current_process<'a>() -> &'a mut Process {
    unsafe {
        PROCESS_LIST.as_mut().unwrap().last_mut().unwrap()
    }
}

pub fn get_parent_process<'a>() -> Option<&'a mut Process> {
    let list = unsafe { PROCESS_LIST.as_mut().unwrap() };
    let n = list.len();
    if n >= 2 {
        Some(&mut list[n-2])
    } else {
        None
    }
}

pub unsafe fn execv(file_name: &str, args: &[&str]) -> usize {
    set_old_process_state_wrapper();

    let process = match loader::create_process(file_name) {
        Ok(process) => process,
        Err(load_error) => match load_error {
            CreationError::ExecutableNotFound => return 0xffffffff,
            CreationError::InvalidElfHeader => return 0xfffffffe,
        }
    };
    add_process(process);

    execv_inner(file_name, args);

    asm!("continue_task_label:" :::: "intel");
    0
}

pub unsafe fn execv_inner(file_name: &str, args: &[&str]) {
    use memory::segmentation::{ SegmentSelector, TableType };
    
    let process = get_current_process();

    let mut arguments = vec![];
    arguments.push(file_name);
    arguments.extend_from_slice(args);
    let load_request = loader::create_load_request(&process, &arguments);
    let load_information = match loader::load_process(&process, &arguments, load_request) {
        Ok(load_information) => load_information,
        Err(load_error) => panic!("Load error: {:?}", load_error),
    };

    process.set_ldt_descriptors(load_information.get_ldt_entries());

    let kernel_stack = 0x9fc00 - (PROCESS_LIST.as_ref().unwrap().len()-1) * 0x20000;
    process.set_kernel_stack(0x10, kernel_stack as u32);

    let code_selector = SegmentSelector::new(0, TableType::LDT, 3);
    let data_selector = SegmentSelector::new(1, TableType::LDT, 3);
    let stack_pointer = (load_information.stack_pointer as usize-8) as *mut u8;
    asm!("
    mov [ebx], $1
    mov [ebx+4], $2
    " ::
    "{ebx}"(load_information.process_base.offset(stack_pointer as isize) as u32),
    "r"(load_information.arguments_count),
    "r"(load_information.argument_pointers_start)
    :: "intel", "volatile");
    
    let entry_point = process.get_elf_header().entry_point as u32;
    let mut init_cpu_state = CpuState::default();
    init_cpu_state.ds = data_selector as u32;
    init_cpu_state.cs = code_selector as u32;
    init_cpu_state.esp = stack_pointer as u32 - 4;
    init_cpu_state.ebp = init_cpu_state.esp;
    init_cpu_state.eip = entry_point;
    init_cpu_state.eflags = 0;
    process.set_cpu_state(init_cpu_state);
    process.set_load_information(load_information);

    context_switch();
}

#[naked]
pub unsafe extern "C" fn set_old_process_state_wrapper() {
    asm!("
    push eax
    push ebx
    push ecx
    push edx
    push esi
    push edi

    mov eax, esp
    add eax, 6*4
    add eax, 1*4 // return address
    push eax

    push ebp

    mov eax, ds
    push eax
    mov eax, cs
    push eax

    pushfd

    lea eax, continue_task_label
    push eax

    mov ebx, $0
    call ebx

    // Dont restore EIP, EFLAGS
    // Dont have to restore CS and DS
    add esp, 4*4
    pop ebp
    // Dont restore esp
    add esp, 4*1
    pop edi
    pop esi
    pop edx
    pop ecx
    pop ebx
    pop eax

    ret"
    :: "m"(set_old_process_state as u32) :: "intel", "volatile");
}

pub unsafe extern "C" fn set_old_process_state(cpu_state: CpuState) {
    if let Some(parent_process) = get_parent_process() {
        // println!("Setting process state for process \"{}\"", parent_process.executable_file.get_process_name());
        // println!("{:?}", &cpu_state);
        parent_process.set_cpu_state(cpu_state);
    }
}

pub unsafe fn context_switch() {
    use memory::gdt::{ Gdt, DescriptorType };
    use memory::GDT;

    let process = get_current_process();

    // Loading LDT and TSS
    GDT.set_ldt(process.get_ldt());
    GDT.set_tss(process.get_tss());
    asm!("lldt $0" :: "r"(GDT.get_selector(DescriptorType::LdtDescriptor, 0) as u16) :: "intel");
    asm!("ltr $0" :: "r"(GDT.get_selector(DescriptorType::TssDescriptor, 0) as u16) :: "intel");

    asm!("mov ebx, $0

    mov eax, [ebx+4*3]  // Data segment
    push eax
    mov eax, [ebx+4*5] // ESP
    push eax
    mov eax, [ebx+4*1]  // Flags
    push eax            
    mov eax, [ebx+4*2]  // Code segment
    push eax
    mov eax, [ebx+4*0]  // EIP
    push eax
   
    mov ecx, [ebx+4*9]
    mov edx, [ebx+4*8]
    mov ebp, [ebx+4*4]
    mov esi, [ebx+4*7]
    mov edi, [ebx+4*6]

    // Pushing ebx
    mov eax, [ebx+4*10]
    push eax

    // Setting segment selectors
    mov eax, [ebx+4*3]
    mov ds, ax
    mov fs, ax
    mov gs, ax
    mov es, ax

    // OS Dev says to restore CR0 too, but it crashes when I does
    // mov eax, [ebx+4*2]
    // mov cr0, eax

    // Setting eax
    mov eax, [ebx+4*11]

    // Setting ebx
    pop ebx

    iretd" ::
    "m"(process.get_cpu_state() as *const CpuState as u32)
    :: "intel", "volatile");
    ::core::intrinsics::unreachable();
}

pub unsafe fn unwind_process() {
    let process = pop_process();
    // Deallocating process
    use alloc::allocator::Layout;
    use alloc::allocator::Alloc;
    let process_base = process.get_load_information().process_base;
    let layout = Layout::from_size_align_unchecked(0, 1);
    (&PROCESS_ALLOCATOR).dealloc(process_base as *mut u8, layout);

    context_switch();
}