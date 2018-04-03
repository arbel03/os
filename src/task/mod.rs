pub mod state;
mod loader;
mod elf;

use self::state::*;
use BitmapAllocator;
use dtables::DescriptorTable;

pub static mut LDT: ::memory::gdt::SegmentDescriptorTable = DescriptorTable::new();
pub static mut TSS: TaskStateSegment = TaskStateSegment::empty();
static mut PROCESS_ALLOCATOR: Option<BitmapAllocator> = None;

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
    use memory::segmentation::*;
    use memory::gdt::{Gdt, DescriptorType};
    use memory::GDT;
    use memory::utils::*;

    let (elf_header, loaded_segments) = loader::load_elf(file_name);

    println!("{:#x}", loaded_segments[1].base);

    // Setting LDT
    LDT.set_descriptors(loaded_segments);
    GDT.set_ldt(&LDT);

    let ldt_selector = GDT.get_selector(DescriptorType::LdtDescriptor, 0);
    let code_selector = SegmentSelector::new(0, TableType::LDT, 3);
    let data_selector = SegmentSelector::new(1, TableType::LDT, 3);
    let stack_selector = SegmentSelector::new(2, TableType::LDT, 3);
    let kernel_data_selector = GDT.get_selector(DescriptorType::KernelData, 0);
    
    // get current stack pointer
    let esp: u32; asm!("mov eax, esp" : "={eax}"(esp) ::: "intel");

    TSS.esp0 = esp;
    TSS.ss0 = kernel_data_selector;
    TSS.eip = elf_header.entry_point;
    TSS.esp = 1024*50;
    TSS.ebp = 1024*50;
    TSS.eflags = 0; // 0x3202;
    TSS.cs = code_selector;
    TSS.ss = stack_selector;
    TSS.ds = data_selector;
    TSS.es = data_selector;
    TSS.gs = data_selector;
    TSS.fs = data_selector;
    TSS.ldtr = ldt_selector;

    // Load LDT
    asm!("lldt $0" :: "r"(ldt_selector as u16) :: "intel", "volatile");

    // Loading TSS
    let tss_selector = GDT.get_selector(DescriptorType::TssDescriptor, 0);
    asm!("ltr $0" :: "r"(tss_selector as u16) :: "intel", "volatile");
    
    load_ds(data_selector);
    load_fs(data_selector);
    load_gs(data_selector);
    load_es(data_selector);

    extern {
        fn context_switch();
    }

    context_switch();
}