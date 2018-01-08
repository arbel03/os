use core::slice;
use core::fmt;
use BootloaderInfo;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MemoryMapEntry {
    base: u64,
    size: u64,
    region_type: u32,
    info: u32,
}

pub fn print_memory_map(bootloader_info: &BootloaderInfo) {
    let ptr = bootloader_info.memory_map_addr as *const MemoryMapEntry;
    let size = bootloader_info.memory_map_count as usize;
    unsafe {
        let slice = slice::from_raw_parts(ptr, size);
        for entry in slice.into_iter() {
            //println!("{:?}", entry);
            println!("Base({:#x}), size({:#x}), type({:#x}), info({:#x})", entry.base, entry.size, entry.region_type, entry.info);
            //println!("Base({:#x}), size({:#x}), type({:#x})", entry.base, entry.size, entry.region_type);
        }
    }
}