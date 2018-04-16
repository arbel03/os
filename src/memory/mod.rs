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
    use self::segmentation::{ SegmentDescriptor, Flags, AccessFlags };
    GDT.init();
    
    // Dividing limit by 4K since granularity flag is on
    let flags = Flags::Size as u8 | Flags::Granularity as u8;
    let limit = free_memory_areas.get_last_address() / 0x1000;

    // Kernel Code Segment
    let access_byte = AccessFlags::PL0 as u8 | AccessFlags::ReadWrite as u8 | AccessFlags::Executable as u8 | AccessFlags::AlwaysOne as u8 | AccessFlags::Present as u8;
    let kernel_code = SegmentDescriptor::new(0, limit, access_byte, flags);
    GDT.set_descriptor(DescriptorType::KernelCode, kernel_code);
    // Kernel Data Segment
    let access_byte = AccessFlags::PL0 as u8 | AccessFlags::ReadWrite as u8 | AccessFlags::Present as u8 | AccessFlags::AlwaysOne as u8;
    let kernel_data = SegmentDescriptor::new(0, limit, access_byte, flags);
    GDT.set_descriptor(DescriptorType::KernelData, kernel_data);

    // Set and load the table
    GDT.load();

    utils::load_ds(GDT.get_selector(DescriptorType::KernelData, 0));
    utils::load_cs(GDT.get_selector(DescriptorType::KernelCode, 0));
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
    use HEAP;

    let mut memory_iter = MemoryMapIterator::new(bootloader_info, MemoryAreaType::Free);

    while let Some(free_memory_area) = memory_iter.next() {
        if free_memory_area.size >= 0x1000 {
            let heap_start = free_memory_area.base as usize;
            let heap_size = free_memory_area.size as usize;

            println!("Setup Heap at {:#08x}, size: {:#08x}", heap_start, heap_size);    
            HEAP.lock().set_bitmap_start(heap_start);
            HEAP.lock().set_block_size(::core::mem::size_of::<usize>()*4);
            HEAP.lock().set_size(heap_size);
            // HEAP.lock().init();
            break;
        }
    }

    let free_memory_areas = get_free_memory_areas(memory_iter, bootloader_info);
    unsafe { setup_descriptors(bootloader_info, &free_memory_areas) };

    // Returning the rest of the free memory areas
    return free_memory_areas;
}