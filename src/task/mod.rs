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

    let mut boxed_process = Box::new(Process::new());

    {
        let process = boxed_process.as_mut();

        // Setting ldt segments
        process.set_ldt_descriptors(loaded_segments);

        let kernel_data_selector = GDT.get_selector(DescriptorType::KernelData, 0);
        let esp0: u32; asm!("mov eax, esp" : "={eax}"(esp0) ::: "intel");

        process.setup_process(kernel_data_selector as u32, esp0, elf_header.entry_point, 1024*50);

        let ldt_selector = GDT.get_selector(DescriptorType::LdtDescriptor, 0);
        let tss_selector = GDT.get_selector(DescriptorType::TssDescriptor, 0);

        // Set GDT
        GDT.set_ldt(process.get_ldt());
        process.get_tss().ldtr = ldt_selector as u32;
        // Set TSS
        GDT.set_tss(process.get_tss());

        asm!("ltr $0" :: "r"(tss_selector as u16) :: "intel");
        asm!("lldt $0" :: "r"(ldt_selector as u16) :: "intel");
    }

    CURRENT_PROCESS = Some(boxed_process);

    load_ds(SegmentSelector::new(1, TableType::LDT, 3));
    load_es(SegmentSelector::new(1, TableType::LDT, 3));
    load_fs(SegmentSelector::new(1, TableType::LDT, 3));
    load_gs(SegmentSelector::new(1, TableType::LDT, 3));

    extern {
        fn context_switch(stack_selector: u32, stack_size: u32, code_selector: u32);
    }

    context_switch(SegmentSelector::new(2, TableType::LDT, 3) as u32, 1024*50 as u32, SegmentSelector::new(0, TableType::LDT, 3) as u32);
}