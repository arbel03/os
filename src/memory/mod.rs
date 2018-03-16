pub mod segmentation;
pub mod memory_map;
pub mod heap;

pub use self::memory_map::{ MemoryAreaType, MemoryArea, MemoryAreas };
use self::memory_map::MemoryMapIterator;
use self::segmentation::*;
use BootloaderInfo;
use alloc::vec::Vec;

extern {
    fn gdt_flush();
}

static mut GDT: DescriptorTable = DescriptorTable::new();

pub fn setup_descriptors(bootloader_info: &BootloaderInfo, free_memory_areas: &MemoryAreas) {
    let mut descriptors: Vec<SegmentDescriptor> = Vec::new();
    // Null descriptor
    descriptors.push(SegmentDescriptor::NULL);
    // Kernel Code Segment
    descriptors.push(SegmentDescriptor::new(0, bootloader_info.kernel_end, 0x9A, 0xC));
    // Kernel Data Segment
    descriptors.push(SegmentDescriptor::new(0, bootloader_info.kernel_end, 0x92, 0xC));
    for memory_area in (&free_memory_areas).0.iter() {
        descriptors.push(SegmentDescriptor::new(memory_area.base as u32, memory_area.size as u32, 0b11111010, 0xC))
    }

    unsafe {
        // Set and load the table
        GDT.set_entries(descriptors);
        GDT.load();
        load_ds(SegmentSelector::new(2, SegmentTable::GDT, 0));
        // TODO: setup stack in a whole different segment to detect stack overflows
        load_ss(SegmentSelector::new(2, SegmentTable::GDT, 0));
        load_cs(SegmentSelector::new(1, SegmentTable::GDT, 0));
    }
}

fn get_free_memory_areas(memory_map: MemoryMapIterator, bootloader_info: &BootloaderInfo) -> MemoryAreas {
    let mut free_memory_areas = MemoryAreas::new();
    let kernel_start = bootloader_info.kernel_start as usize;
    let kernel_end = bootloader_info.kernel_end as usize;

    for memory_area in memory_map {
        if kernel_start >= memory_area.base {
            let memory_area_end = memory_area.base + memory_area.size;
            if kernel_end > memory_area_end {
                // Adding a memory area from the base address to the kernel start
                free_memory_areas.insert(MemoryArea::new(memory_area.base, kernel_start-memory_area.base));
                continue;
            } else if kernel_end < memory_area_end {
                // Need to add two memory areas, one before and one after the kernel
                if memory_area.base != kernel_start {
                    free_memory_areas.insert(MemoryArea::new(memory_area.base, kernel_start-memory_area.base));
                }
                free_memory_areas.insert(MemoryArea::new(kernel_end, memory_area_end-kernel_end));
                continue;
            }
        }
        free_memory_areas.insert(memory_area);
    }
    return free_memory_areas;
}

pub fn init(bootloader_info: &BootloaderInfo) -> MemoryAreas {
    use core::cmp::min;
    use HEAP;

    let mut memory_iter = MemoryMapIterator::new(&bootloader_info, MemoryAreaType::Free);
    let first_free_memory_area = memory_iter.next().unwrap();
    let heap_start = first_free_memory_area.base as usize;
    let heap_size = min(first_free_memory_area.size, 1024*1000) as usize;
    (*HEAP).lock().set_bitmap_start(heap_start);
    (*HEAP).lock().set_size(heap_size);
    println!("Setup Heap at {:#08x}, size: {:#08x}", heap_start, heap_size);

    let free_memory_areas = get_free_memory_areas(memory_iter, bootloader_info);
    setup_descriptors(bootloader_info, &free_memory_areas);

    // Returning the rest of the free memory areas
    return free_memory_areas;
}