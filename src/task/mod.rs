pub mod state;
mod loader;
mod elf;

use self::state::*;
use BitmapAllocator;
use memory::{ MemoryAreas, gdt };
use dtables::DescriptorTable;

pub static mut LDT: gdt::SegmentDescriptorTable = DescriptorTable::new();
pub static mut TSS: TaskStateSegment = TaskStateSegment::empty();
static mut PROCESS_ALLOCATOR: Option<BitmapAllocator> = None;

pub fn init(free_memory_areas: MemoryAreas) {
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
    //LDT.set_entries(loaded_segments);
    
    GDT.set_ldt(&LDT);

    // Set up TSS
    TSS.ss0 = SegmentSelector::new(2, TableType::GDT, 0).0 as u32;
    TSS.esp0 = 0x9fc00;
    TSS.eip = elf_header.entry_point;
    TSS.cs = SegmentSelector::new(1, TableType::LDT, 3).0 as u32;
    TSS.ds = SegmentSelector::new(2, TableType::LDT, 3).0 as u32;
    TSS.ss = SegmentSelector::new(3, TableType::LDT, 3).0 as u32;
    TSS.esp = 0;
    TSS.ebp = 0;
    TSS.ldtr = SegmentSelector::new(4, TableType::GDT, 3).0 as u32;
    TSS.iopb_offset = 104;

    // Getting the ldt selector
    // let ldt_selector = GDT.get_selector(SegmentType::LdtDescriptor, 0);
    // asm!("mov ax, $0; lldt ax" :: "m"(ldt_selector.0 as u32) :: "intel", "volatile");
    
    // let tss_selector = GDT.get_selector(SegmentDescriptorType::TssDescriptor, 3);
    // asm!("mov ax, $0; ltr ax" :: "m"(tss_selector.0 as u32) :: "intel", "volatile");

    // println!("entry_point: {:#x}", elf_header.entry_point);
    // asm!("
    // mov ax, $0
    // jmp ax:$1" :: "m"(tss_selector.0 as u16), "m"(elf_header.entry_point) :: "intel", "volatile");

    let ss: u16;
    asm!("mov ax, ss" : "={ax}"(ss) ::: "intel");
    println!("ss: {:#x}", ss);
}