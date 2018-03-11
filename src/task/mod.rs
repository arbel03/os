pub mod loader;
use memory::{ MemoryAreas };
mod elf;

pub fn init(free_memory_areas: MemoryAreas) {
    for free_memory_area in free_memory_areas.0.iter() {
        println!("From {:#x} to {:#x}.", free_memory_area.base as usize, (free_memory_area.base+free_memory_area.size) as usize);
    }
}