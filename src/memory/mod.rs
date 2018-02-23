pub mod segmentation;
pub mod memory_map;
pub mod heap;

use BootloaderInfo;
use alloc::vec::Vec;
use self::memory_map::{ MemoryAreaIterator, MemoryAreaType };
use self::segmentation::*;

extern {
    fn gdt_flush();
}

static mut GDT: Option<DescriptorTable> = None;

pub fn setup_descriptors(bootloader_info: &BootloaderInfo) {
    let mut descriptors: Vec<SegmentDescriptor> = Vec::new();
    // Null descriptor
    descriptors.push(SegmentDescriptor::NULL);
    // Kernel Code Segment
    descriptors.push(SegmentDescriptor::new(0, bootloader_info.kernel_end, 0x9A, 0xC));
    // Kernel Data Segment
    descriptors.push(SegmentDescriptor::new(0, bootloader_info.kernel_end, 0x92, 0xC));

    unsafe {
        // Initialize a new DescriptorTable
        GDT = Some(DescriptorTable::new());
        // Get a reference to the boxed value that DescriptorTable holds
        let table = GDT.as_mut().unwrap();
        // Set and load the table
        table.set_entries(descriptors);
        table.load();
        gdt_flush();
    }
}

pub fn init(bootloader_info: &BootloaderInfo) {
    use core::cmp::min;
    use HEAP;

    let mut memory_iter = MemoryAreaIterator::new(&bootloader_info, MemoryAreaType::Free);
    let first_free_memory_area = memory_iter.next().unwrap();
    let heap_start = first_free_memory_area.base as usize;
    let heap_size = min(first_free_memory_area.size, 1024*1000) as usize;
    (*HEAP).lock().set_bitmap_start(heap_start);
    (*HEAP).lock().set_size(heap_size);
    println!("Setup Heap at {:#08x}, size: {:#08x}", heap_start, heap_size);

    setup_descriptors(bootloader_info);
}