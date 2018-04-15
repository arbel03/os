pub mod heap;
pub mod gdt;
pub mod segmentation;
pub mod utils;
mod memory_map;

pub use self::memory_map::*;
use BootloaderInfo;
use dtables;

pub static mut GDT: gdt::SegmentDescriptorTable = dtables::DescriptorTable::new();

pub unsafe fn setup_descriptors(_bootloader_info: &BootloaderInfo, free_memory_areas: &MemoryAreas) {
    use self::gdt::{ DescriptorType, Gdt };
    use self::segmentation::SegmentDescriptor;
    GDT.init();
    
    // Kernel Code Segment
    GDT.set_descriptor(DescriptorType::KernelCode, SegmentDescriptor::new(0, free_memory_areas.get_last_address(), 0x9A, 0b0100));
    // Kernel Data Segment
    GDT.set_descriptor(DescriptorType::KernelData, SegmentDescriptor::new(0, free_memory_areas.get_last_address(), 0x92, 0b0100));

    // Set and load the table
    GDT.load();

    utils::load_ds(GDT.get_selector(DescriptorType::KernelData, 0));
    utils::load_cs(GDT.get_selector(DescriptorType::KernelCode, 0));
    // TODO: setup stack in a whole different segment to detect stack overflows
    utils::load_ss(GDT.get_selector(DescriptorType::KernelData, 0));
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
    // FIXME: Crashing when initializing heap here.
    // (*HEAP).lock().init();
    println!("Setup Heap at {:#08x}, size: {:#08x}", heap_start, heap_size);

    let free_memory_areas = get_free_memory_areas(memory_iter, bootloader_info);
    unsafe { setup_descriptors(bootloader_info, &free_memory_areas) };

    // Returning the rest of the free memory areas
    return free_memory_areas;
}