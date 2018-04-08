pub mod process;
mod loader;
mod elf;

use BitmapAllocator;
use self::process::*;
use dtables::DescriptorTable;
use alloc::boxed::Box;

static mut PROCESS_ALLOCATOR: Option<BitmapAllocator> = None;
pub static mut CURRENT_PROCESS: Option<Box<Process>> = None;

pub fn init(free_memory_areas: ::memory::MemoryAreas) {
    // Set up an allocator for the process area
    let process_area = free_memory_areas.0[0];
    println!("Allocating processes from {:#x} to {:#x}.", process_area.base, process_area.base+process_area.size);
    unsafe {
        PROCESS_ALLOCATOR = Some(BitmapAllocator::new(process_area.base, process_area.size, process_area.size/1000));
        PROCESS_ALLOCATOR.as_mut().unwrap().init();
    }
}

pub unsafe fn execv(file_name: &str) {
    use memory::utils::*;
    use memory::segmentation::{ SegmentSelector, TableType };
    use memory::gdt::{ Gdt, DescriptorType };
    use memory::GDT;

    let (elf_header, loaded_segments) = loader::load_elf(file_name);


    // If only 3 segments, one is stack and the other is a null segment.
    // Every task should have one code segment.
    if loaded_segments.len() < 3 {
        panic!("Must have atleast one Code Segment");
    }

    let mut boxed_process = Box::new(Process::new());

    let (code_selector, data_selector, stack_selector) = {
        let process = boxed_process.as_mut();

        let code_selector = SegmentSelector::new(1, TableType::LDT, 3);
        // Adding stack segment last in `load_elf`
        let stack_selector = SegmentSelector::new(loaded_segments.len()-1, TableType::LDT, 3);
        let data_selector = SegmentSelector::new(if loaded_segments.len() == 3 { 2 } else { 0 }, TableType::LDT, 3);

        // Setting process data
        process.setup_process(GDT.get_selector(DescriptorType::KernelData, 0), 0x9fc00, elf_header.entry_point, 1024*50, code_selector, data_selector, stack_selector);
        
        // Setting ldt segments
        process.set_ldt_descriptors(loaded_segments);

        // Set GDT
        GDT.set_ldt(process.get_ldt());
        process.get_tss().ldtr = GDT.get_selector(DescriptorType::LdtDescriptor, 0) as u32;
        // Set TSS
        GDT.set_tss(process.get_tss());

        asm!("ltr $0" :: "r"(GDT.get_selector(DescriptorType::TssDescriptor, 0) as u16) :: "intel");
        asm!("lldt $0" :: "r"(GDT.get_selector(DescriptorType::LdtDescriptor, 0) as u16) :: "intel");

        (code_selector, data_selector, stack_selector)
    };

    CURRENT_PROCESS = Some(boxed_process);

    load_ds(data_selector);
    load_es(data_selector);
    load_fs(data_selector);
    load_gs(data_selector);
    
    extern {
        fn context_switch(stack_selector: u32, stack_size: u32, code_selector: u32);
    }

    context_switch(stack_selector as u32, 1024*50 as u32, code_selector as u32);
}