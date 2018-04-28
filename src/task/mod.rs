pub mod process;
mod loader;
mod elf;

use self::process::*;
use BitmapAllocator;
use alloc::boxed::Box;
use alloc::Vec;
use memory::MemoryArea;

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

pub fn pop_process() {
    unsafe {
        PROCESS_LIST.as_mut().unwrap().pop();
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

pub unsafe fn execv(file_name: &str, args: &[&str]) {
    use memory::segmentation::{ SegmentSelector, TableType };

    let process = loader::create_process(file_name).expect("An error occured.");
    add_process(process);
    
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

    let kernel_stack = 0x9fc00 - (PROCESS_LIST.as_ref().unwrap().len()-1) * 0x10000;
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
    :: "intel");
    
    let entry_point = process.get_elf_header().entry_point as u32;
    let mut init_cpu_state = CpuState::default();
    init_cpu_state.ds = data_selector as u32;
    init_cpu_state.fs = data_selector as u32;
    init_cpu_state.es = data_selector as u32;
    init_cpu_state.gs = data_selector as u32;
    init_cpu_state.ss = data_selector as u32;

    init_cpu_state.cs = code_selector as u32;
    init_cpu_state.esp = stack_pointer as u32 - 4;
    init_cpu_state.ebp = init_cpu_state.esp;
    init_cpu_state.eip = entry_point;
    init_cpu_state.eflags = 0;
    process.set_cpu_state(init_cpu_state);
    process.set_load_information(load_information);

    switch_process_new_wrapper();
}

#[naked]
pub unsafe fn switch_process_new_wrapper() {
    asm!("
    push gs
    push fs
    push ds
    push ss
    push cs
    push es
    push edi
    push esi
    push ebp
    mov edi, esp
    push edi
    push ebx
    push edx
    push ecx
    push eax
    pushfd
    mov eax, .continue_task
    push eax
    
    mov eax, esp
    push eax

    mov ebx, $0
    call ebx

    add esp, 16*4 
.continue_task:
    ret
    "
    :: "m"(switch_process_new as u32) :: "intel");
}

pub unsafe fn switch_process_new(cpu_state: &CpuState) {
    if let Some(parent_process) = get_parent_process() {
        println!("Setting process state for process \"{}\"", parent_process.executable_file.get_process_name());
        println!("{:?}", cpu_state);
        parent_process.set_cpu_state(cpu_state.clone());
    }
    context_switch();
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

    // println!("Context switch to: {:?}", process.get_cpu_state());

    asm!("mov ebx, $0

    push [ebx+4*13]
    push [ebx+4*6]
    pushfd
    push [ebx+4*11]
    push [ebx+4*0]

    mov eax, [ebx+4*13]
    mov ds, ax
    mov fs, ax
    mov gs, ax
    mov es, ax

    mov ecx, [ebx+4*3]
    mov edx, [ebx+4*4]
    mov esi, [ebx+4*8]
    mov edi, [ebx+4*9]
    mov eax, [ebx+4*7]
    mov ebp, eax
    mov eax, [ebx+4*5]
    push eax
    mov eax, [ebx+4*2]
    pop ebx
    iretd
    " ::
    "r"(process.get_cpu_state() as *const CpuState)
    :: "intel", "volatile");
    ::core::intrinsics::unreachable();
}

pub unsafe fn unwind_process() {
    println!("Performing unwind context switch.");
    pop_process();
    context_switch();
}