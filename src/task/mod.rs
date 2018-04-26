pub mod process;
mod loader;
mod elf;

use self::process::*;
use BitmapAllocator;
use alloc::boxed::Box;
use alloc::Vec;
use memory::MemoryArea;

static mut PROCESS_ALLOCATOR: BitmapAllocator = BitmapAllocator::new(0x0, 0x0, 0x0);
pub static mut CURRENT_PROCESS: Option<Box<Process>> = None;

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
        }
    } else {
        panic!("No space for process allocator.");
    }
}

pub unsafe fn execv(file_name: &str, args: &[&str]) {
    use memory::segmentation::{ SegmentSelector, TableType };
    use core::ops::DerefMut;

    let mut process = loader::create_process(file_name).expect("An error occured.");

    CURRENT_PROCESS = {
        use core::ptr;
        match ptr::read(&CURRENT_PROCESS) {
            Some(previous_process) => process.set_parent_process(previous_process),
            _ => (),
        }
        Some(Box::new(process))
    };
    let process = CURRENT_PROCESS.as_mut().unwrap().deref_mut();
    
    let mut arguments = vec![];
    arguments.push(file_name);
    arguments.extend_from_slice(args);
    let load_request = loader::create_load_request(&process, &arguments);
    let load_information = match loader::load_process(&process, &arguments, load_request) {
        Ok(load_information) => load_information,
        Err(load_error) => panic!("Load error: {:?}", load_error),
    };

    process.set_ldt_descriptors(load_information.get_ldt_entries());
    process.set_kernel_stack(0x10, 0x9fc00);

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
    init_cpu_state.cs = code_selector as u32;
    init_cpu_state.esp = stack_pointer as u32 - 4;
    init_cpu_state.eip = entry_point;
    process.set_cpu_state(init_cpu_state);
    process.set_load_information(load_information);

    context_switch(process);
}

pub unsafe fn context_switch(process: &Process) {
    use memory::gdt::{ Gdt, DescriptorType };
    use memory::GDT;

    // Loading LDT and TSS
    GDT.set_ldt(process.get_ldt());
    GDT.set_tss(process.get_tss());
    asm!("lldt $0" :: "r"(GDT.get_selector(DescriptorType::LdtDescriptor, 0) as u16) :: "intel");
    asm!("ltr $0" :: "r"(GDT.get_selector(DescriptorType::TssDescriptor, 0) as u16) :: "intel");
    // Perform context switch to loaded task.
    asm!("
    push $0
    push $1
    pushfd
    push $2
    push $3
    mov eax, $0
    mov ds, ax
    mov fs, ax
    mov gs, ax
    mov es, ax
    iretd
    " ::
    "r"(process.get_cpu_state().ds),
    "r"(process.get_cpu_state().esp),
    "r"(process.get_cpu_state().cs),
    "r"(process.get_cpu_state().eip)
    :: "intel", "volatile");
}