use BootloaderInfo;
use core::slice;
use core::cmp::Ordering;
use alloc::Vec;

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

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct MemoryMapEntry {
    pub base: u64,
    pub size: u64,
    pub region_type: u32,
    _acpi_info: u32,
}

#[derive(Clone, Copy)]
pub struct MemoryArea {
    pub base: usize,
    pub size: usize,
}

impl MemoryArea {
    pub fn from(memory_map_entry: MemoryMapEntry) -> Self {
        MemoryArea {
            base: memory_map_entry.base as usize,
            size: memory_map_entry.size as usize,
        }
    }

    pub fn new(base: usize, size: usize) -> Self {
        MemoryArea {
            base: base,
            size: size,
        }
    }
}

impl Eq for MemoryArea {

}

impl Ord for MemoryArea {
    fn cmp(&self, other: &MemoryArea) -> Ordering {
        self.base.cmp(&other.base)
    }
}

impl PartialOrd for MemoryArea {
    fn partial_cmp(&self, other: &MemoryArea) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MemoryArea {
    fn eq(&self, other: &MemoryArea) -> bool {
        self.base == other.base
    }
}

pub struct MemoryAreas(pub Vec<MemoryArea>);

impl MemoryAreas {
    pub fn new() -> Self {
        MemoryAreas(Vec::new())
    }

    pub fn insert(&mut self, memory_area: MemoryArea) {
        match self.0.binary_search(&memory_area) {
            Ok(_) => {} // element already in vector @ `pos` 
            Err(pos) => self.0.insert(pos, memory_area),
        }  
    }

    pub fn get_last_address(&self) -> u32 {
        let mut max = 0;
        for area in self.0.iter() {
            let curr = area.base+area.size;
            if curr > max {
                max = curr;
            }
        }
        return max as u32;
    }
}

pub(in super) struct MemoryMapIterator {
    region_type: MemoryAreaType,
    memory_map: &'static [MemoryMapEntry],
    i: usize,
}

// An iterator that returns only memory map entries with the given type
impl MemoryMapIterator {
    pub fn new(bootloader_info: &BootloaderInfo, _type: MemoryAreaType) -> MemoryMapIterator {
        let ptr = bootloader_info.memory_map_addr as *const MemoryMapEntry;
        let size = bootloader_info.memory_map_count as usize;
        unsafe {
            // Create slice from ptr(0x500, start of memory map) and size
            let slice = slice::from_raw_parts(ptr, size);
            MemoryMapIterator {
                region_type: _type,
                memory_map: slice,
                i: 0,
            }
        }
    }
}

impl Iterator for MemoryMapIterator {
    type Item = MemoryArea;    

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.memory_map.len() {
            let entry = self.memory_map[self.i];
            self.i += 1;
            if entry.region_type == self.region_type as u32 || self.region_type == MemoryAreaType::All {
                return Some(MemoryArea::from(entry));
            }
        }
        None
    }
}