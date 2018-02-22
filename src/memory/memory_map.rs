use BootloaderInfo;
use core::slice;

#[repr(usize)]
#[derive(PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum MemoryAreaType {
    Free = 1,
    Reserved = 2,
    AcpiReclaimable = 3,
    AcpiNonVolatile = 4,
    Bad = 5,
    All = 0xff,
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct MemoryMapEntry {
    pub base: u64,
    pub size: u64,
    pub region_type: u32,
    acpi_info: u32,
}

pub struct MemoryAreaIterator {
    region_type: MemoryAreaType,
    memory_map: &'static [MemoryMapEntry],
    i: usize,
}

// An iterator that returns only memory map entries with the given type
impl MemoryAreaIterator {
    pub fn new(bootloader_info: &BootloaderInfo, _type: MemoryAreaType) -> MemoryAreaIterator {
        let ptr = bootloader_info.memory_map_addr as *const MemoryMapEntry;
        let size = bootloader_info.memory_map_count as usize;
        unsafe {
            // Create slice from ptr(0x500, start of memory map) and size
            let slice = slice::from_raw_parts(ptr, size);
            MemoryAreaIterator {
                region_type: _type,
                memory_map: slice,
                i: 0,
            }
        }
    }
}

impl Iterator for MemoryAreaIterator {
    type Item = MemoryMapEntry;    

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.memory_map.len() {
            let entry = self.memory_map[self.i];
            self.i += 1;
            if entry.region_type == self.region_type as u32 || self.region_type == MemoryAreaType::All {
                return Some(entry);
            }
        }
        None
    }
}