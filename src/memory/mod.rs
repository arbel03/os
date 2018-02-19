pub mod segmentation;
pub mod memory_map;
pub mod heap;

use BootloaderInfo;
use self::memory_map::MemoryAreaIterator;
use self::segmentation::*;
use dtables::{TableDescriptor, lgdt};

extern {
    fn gdt_flush();
}

// My gdt, containing 8 bytes entries (or unsigned 64 bit values)
static mut GDT: [u64; 8] = [0; 8];

unsafe fn encode_entry_at(index: usize, entry: SegmentDescriptor) {
    let mut descriptor_high: u32;
    // Create the high 32 bit segment
    descriptor_high  =  (entry.base & 0x00FF0000) >> 16;
    descriptor_high |= (entry.access_byte as u32) << 8;
    descriptor_high |= entry.limit & 0x000F0000;
    descriptor_high |=  (entry.flags as u32) << 20;
    descriptor_high |= entry.base & 0xFF000000;        

    let mut descriptor_low: u32 = 0;
    // Create the low 32 bit segment
    descriptor_low |= entry.base << 16;                 // set base bits 15:0
    descriptor_low |= entry.limit & 0xFFFF;               // set limit bits 15:0

    GDT[index] = (descriptor_high as u64) << 32;
    GDT[index] |= descriptor_low as u64;
}

pub fn init(bootloader_info: &BootloaderInfo) {
    // Adding gdt entries
    let code_segment = SegmentDescriptor::new(0, 0xffffffff, 0x9A, 0xC);
    let data_segment = SegmentDescriptor::new(0, 0xffffffff, 0x92, 0xC);
    
    unsafe {
        encode_entry_at(0, SegmentDescriptor::NULL);
        encode_entry_at(1, code_segment);
        encode_entry_at(2, data_segment);

        // Loading gdt
        let gdtr = TableDescriptor::new(&GDT);
        lgdt(&gdtr);
        gdt_flush();
    }

    println!("Printing GDT");
    for entry in [code_segment, data_segment].iter() {
        println!("  {:?}", entry);
    }

    println!("Printing free memory areas");
    let memory_iter = MemoryAreaIterator::new(&bootloader_info, 0x1);
    for memory_area in memory_iter {
        unsafe {
            println!("  Start: {:#x}, End: {:#x}, Size: {:#x}", memory_area.base, memory_area.base + memory_area.size, memory_area.size);
        }
    }
}