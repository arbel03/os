pub mod loader;
use BitmapAllocator;
use memory::{ MemoryAreas };
mod elf;

static mut PROCESS_ALLOCATOR: Option<BitmapAllocator> = None;

pub fn init(free_memory_areas: MemoryAreas) {
    let process_area = free_memory_areas.0[0];
    println!("Allocating processes from {:#x} to {:#x}.", process_area.base, process_area.base+process_area.size);
    unsafe {
        PROCESS_ALLOCATOR = Some(BitmapAllocator::new(process_area.base, process_area.size, process_area.size/500));
        PROCESS_ALLOCATOR.as_mut().unwrap().init();
    }
}