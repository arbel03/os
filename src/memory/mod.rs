pub mod heap;
pub mod gdt;
pub mod segmentation;
pub mod utils;
mod memory_map;

pub use self::memory_map::*;
use BootloaderInfo;
use dtables;

pub static mut GDT: gdt::SegmentDescriptorTable = dtables::DescriptorTable::new();

pub unsafe fn setup_descriptors(_bootloader_info: &BootloaderInfo) {
    use self::gdt::{ DescriptorType, Gdt };
    use self::segmentation::{ SegmentDescriptor, Flags, AccessFlags };
    GDT.init();
    
    // Dividing limit by 4K since granularity flag is on
    let flags = Flags::Size as u8; // | Flags::Granularity as u8;
    let limit = 0x13e9a3;

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

pub fn get_free_areas(current_memory_map: &[MemoryArea; 10], occupied_area_iter: &mut ::core::slice::Iter<MemoryArea>) -> [MemoryArea; 10] {
    if let Some(current_occupied_area) = occupied_area_iter.next() {
        let mut new_memory_map = [MemoryArea::Empty; 10];
        let mut insertion_index = 0;
        for area in current_memory_map {
            let result = MemoryArea::from(area.clone()).subtract(current_occupied_area);
            if let Some(before) = result.0 {
                new_memory_map[insertion_index] = before;
                insertion_index += 1;
            }

            if let Some(after) = result.1 {
                new_memory_map[insertion_index] = after;
                insertion_index += 1;
            }

            if result.0.is_none() && result.1.is_none() {
                new_memory_map[insertion_index] = area.clone();
                insertion_index += 1;
            }
            // println!("{:#x} -> {:#x}", (area.base) as u32, (area.base + area.size) as u32);
        }
        return get_free_areas(&new_memory_map, occupied_area_iter);
    } else {
        return current_memory_map.clone();
    }
}

use alloc::Vec;
pub fn init(bootloader_info: &BootloaderInfo) -> Vec<MemoryArea> {
    use HEAP;
    
    println!("Kernel loaded from {:#x} to {:#x}", bootloader_info.kernel_start, bootloader_info.kernel_end);

    let kernel_area = MemoryArea::new(bootloader_info.kernel_start as usize, bootloader_info.kernel_end as usize - bootloader_info.kernel_start as usize);
    let memory_map = MemoryMap::new(bootloader_info);
    
    let mut free_memory_areas = [MemoryArea::Empty; 10];
    let mut current_index = 0;
    for entry in memory_map.memory_map.iter() {
        if entry.get_region_type() as u32 == MemoryAreaType::Free as u32 {
            free_memory_areas[current_index] = MemoryArea::from(entry.clone());
            current_index += 1;
        }
    }

    let occupied_areas = [kernel_area];
    let free_memory_areas = get_free_areas(&free_memory_areas, &mut occupied_areas.iter());

    let heap_memory_area = free_memory_areas[0];
    if heap_memory_area.size >= 0x1000 {
        let heap_start = heap_memory_area.base as usize;
        let heap_size = heap_memory_area.size as usize;

        println!("Setup Heap at {:#08x}, size: {:#08x}", heap_start, heap_size);    
        HEAP.lock().set_bitmap_start(heap_start);
        HEAP.lock().set_block_size(::core::mem::size_of::<usize>()*4);
        HEAP.lock().set_size(heap_size);
        HEAP.lock().init();
    }

    unsafe { setup_descriptors(bootloader_info) };

    let mut _free_memory_areas: Vec<MemoryArea> = Vec::new();
    for area in &free_memory_areas[1..] {
        _free_memory_areas.push(area.clone());
    }

    // Returning the rest of the free memory areas
    return _free_memory_areas;
}