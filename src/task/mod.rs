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
        PROCESS_ALLOCATOR = Some(BitmapAllocator::new(process_area.base, process_area.size, process_area.size/500));
        PROCESS_ALLOCATOR.as_mut().unwrap().init();
    }
}

pub unsafe fn execv(file_name: &str) {
    use memory::segmentation::*;
    use memory::gdt::Gdt;
    use memory::GDT;

    let (elf_header, loaded_segments) = loader::load_elf(file_name);
    
    // Setting LDT
    LDT.set_descriptors(loaded_segments);
    GDT.set_ldt(&LDT);

    // Load LDT
    let ldt_selector = GDT.get_selector(SegmentType::LdtDescriptor, 0);
    asm!("mov ax, $0; lldt ax" :: "m"(ldt_selector) :: "intel", "volatile");

    TSS.prev_tss = 0;
    TSS.esp0 = 0x9fc00;
    TSS.ss0 = GDT.get_selector(SegmentType::KernelData, 0);
    TSS.esp1 = 0;
    TSS.ss1 = 0;
    TSS.esp2 = 0;
    TSS.ss2 = 0;
    TSS.cr3 = 0;
    TSS.eip = elf_header.entry_point;
    TSS.eflags = 0;
    TSS.eax = 0;
    TSS.ecx = 0;
    TSS.edx = 0;
    TSS.ebx = 0;
    TSS.esp = 0;
    TSS.ebp = 0;
    TSS.esi = 0;
    TSS.edi = 0;
    TSS.es = SegmentSelector::new(2, TableType::LDT, 3);
    TSS.cs = SegmentSelector::new(1, TableType::LDT, 3);
    TSS.ss = SegmentSelector::new(3, TableType::LDT, 3);
    TSS.ds = SegmentSelector::new(2, TableType::LDT, 3);
    TSS.fs = SegmentSelector::new(2, TableType::LDT, 3);
    TSS.gs = SegmentSelector::new(2, TableType::LDT, 3);
    TSS.ldtr = GDT.get_selector(SegmentType::LdtDescriptor, 0);
    TSS.res = 0;
    TSS.iopb_offset = 104;


    // Loaded TSS
    let tss_selector = GDT.get_selector(SegmentType::TssDescriptor, 3);
    asm!("mov ax, $0; ltr ax" :: "m"(tss_selector) :: "intel", "volatile");

    //asm!("ljmp 0b11000, 0" :::: "intel");
}