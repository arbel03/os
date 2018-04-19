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
    use memory::gdt::{ Gdt, DescriptorType };
    use memory::GDT;
    
    let mut process = loader::create_process(file_name).expect("An error occured.");

    let load_request = loader::create_load_request(&process, args);
    let load_information = match loader::load_process(&process, args, load_request) {
        Ok(load_information) => load_information,
        Err(load_error) => panic!("Load error: {:?}", load_error),
    };

    let code_selector = SegmentSelector::new(0, TableType::LDT, 3);
    let data_selector = SegmentSelector::new(1, TableType::LDT, 3);
    let stack_pointer = load_information.stack_pointer as u32;
    asm!("
    mov eax, esp
    mov esp, $0
    push $1
    push $2
    mov esp, eax
    " ::
    "{ebx}"(stack_pointer as u32),
    "{ecx}"(load_information.argument_pointers_start as u32 - load_information.process_base as u32),
    "{edx}"(args.len())
    :: "intel");

    let virtual_stack_pointer = load_information.translate_physical_to_virtual_address((stack_pointer-12) as *const u8);
    let elf_header = process.get_elf_header();
    // Set process information
    process.setup_process(GDT.get_selector(DescriptorType::KernelData, 0), 0x9fc00);
    // Set LDT in GDT
    process.set_ldt_descriptors(load_information.get_ldt_entries());
    GDT.set_ldt(process.get_ldt());
    // Set TSS
    process.get_tss().ldtr = GDT.get_selector(DescriptorType::LdtDescriptor, 0) as u32;
    GDT.set_tss(process.get_tss());

    asm!("ltr $0" :: "r"(GDT.get_selector(DescriptorType::TssDescriptor, 0) as u16) :: "intel");
    asm!("lldt $0" :: "r"(GDT.get_selector(DescriptorType::LdtDescriptor, 0) as u16) :: "intel");

    process.set_load_information(load_information);

    CURRENT_PROCESS = Some(Box::new(process));

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
    "r"(data_selector as u32),
    "r"(virtual_stack_pointer as u32),
    "r"(code_selector as u32)
    "r"(elf_header.entry_point as u32)
    :: "intel", "volatile");
}
