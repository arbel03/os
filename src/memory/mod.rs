pub mod segmentation;
pub mod memory_map;
pub mod heap;

use BootloaderInfo;
use alloc::vec::Vec;
use alloc::boxed::Box;
use self::memory_map::{ MemoryAreaIterator, MemoryAreaType };
use self::segmentation::*;
use dtables::{ TableDescriptor, lgdt };

extern {
    fn gdt_flush();
}

type GdtEntry = u64;

unsafe fn encode_entry(entry: SegmentDescriptor) -> GdtEntry {
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

    let mut descriptor: u64 = (descriptor_high as u64) << 32;
    descriptor |= descriptor_low as u64;
    return descriptor;
}

static mut GDT: Option<Box<[GdtEntry]>> = None;
pub unsafe fn set_gdt(descriptors: Vec<SegmentDescriptor>) {
    let mut gdt: Vec<u64> = Vec::with_capacity(descriptors.len()); 
    for descriptor in descriptors {
        gdt.push(encode_entry(descriptor));
    }
    
    GDT = Some(gdt.into_boxed_slice());
    use core::ops::Deref;
    let gdt_entries = GDT.as_ref().unwrap().deref();

    // Printing Gdt    
    // for entry in gdt_entries.iter() {
    //     println!("{:#x}{:x}", *entry >> 32 as u32, *entry as u32);
    // }

    // Loading gdt
    let gdtr = TableDescriptor::new(gdt_entries);
    println!("Gdtr: {:?}", gdtr);
    lgdt(&gdtr);
    gdt_flush();
}

pub fn init(bootloader_info: &BootloaderInfo) {
    let mut descriptors: Vec<SegmentDescriptor> = Vec::new();
    // Null descriptor
    descriptors.push(SegmentDescriptor::NULL);
    // Kernel Code Segment
    descriptors.push(SegmentDescriptor::new(0, bootloader_info.kernel_end, 0x9A, 0xC));
    // Kernel Data Segment
    descriptors.push(SegmentDescriptor::new(0, bootloader_info.kernel_end, 0x92, 0xC));

    unsafe {
        set_gdt(descriptors);
    }

    println!("Printing free memory areas");
    let memory_iter = MemoryAreaIterator::new(&bootloader_info, MemoryAreaType::Free);
    for memory_area in memory_iter {
        unsafe {
            println!("  Start: {:#010x}, Size: {:#010x}, Type: {}", memory_area.base, memory_area.size, memory_area.region_type);
        }
    }
}