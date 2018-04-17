use BootloaderInfo;
use core::slice;
use alloc::Vec;
use core::fmt;

#[allow(dead_code)]
#[repr(u32)]
#[derive(Debug)]
pub enum MemoryAreaType {
    Free = 1,
    Reserved = 2,
    AcpiReclaimable = 3,
    AcpiNonVolatile = 4,
    Bad = 5,
}

impl From<u32> for MemoryAreaType {
    fn from(x: u32) -> Self {
        match x {
            1 => MemoryAreaType::Free,
            2 => MemoryAreaType::Reserved,
            3 => MemoryAreaType::AcpiNonVolatile,
            4 => MemoryAreaType::AcpiReclaimable,
            5 => MemoryAreaType::Bad,
            _ => panic!("Value {} is not a variation of enum MemoryAreaType.", x),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct MemoryMapEntry {
    pub base: u64,
    pub size: u64,
    region_type: u32,
    acpi_info: u32,
}

impl MemoryMapEntry {
    pub fn get_base(&self) -> u64 {
        self.base
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_region_type(&self) -> MemoryAreaType {
        MemoryAreaType::from(self.region_type)
    }
}

impl fmt::Display for MemoryMapEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MemoryMapEntry {{\n\tbase: {:#x}\n\tsize: {:#x}\n\ttype: {:?}\n}}", self.base, self.size, self.get_region_type())
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MemoryArea {
    pub base: usize,
    pub size: usize,
}

impl MemoryArea {
    pub const Empty: MemoryArea = MemoryArea {
        base: 0,
        size: 0,
    };

    pub fn new(base: usize, size: usize) -> Self {
        MemoryArea {
            base: base,
            size: size,
        }
    }

    pub fn subtract(&self, other: &MemoryArea) -> (Option<MemoryArea>, Option<MemoryArea>) {
        let mut before = None;
        let mut after = None;

        if self.base < other.base {
            use core::cmp::min;
            before = Some(MemoryArea::new(self.base, min(other.base - self.base, self.size)));
        }

        let other_end = other.base + other.size;
        let end = self.base + self.size;
        if end > other_end {
            use core::cmp::max;
            after = Some(MemoryArea::new(max(other_end, self.base), end - other_end));
        }

        return (before, after)
    }
}

impl fmt::Display for MemoryArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MemoryArea {{\n\tbase: {:#x}\n\tsize: {:#x}\n}}", self.base, self.size)
    }
}

impl From<MemoryMapEntry> for MemoryArea {
    fn from(memory_map_entry: MemoryMapEntry) -> Self {
        MemoryArea {
            base: memory_map_entry.base as usize,
            size: memory_map_entry.size as usize,
        }
    }
}

pub struct MemoryMap<'a> {
    pub memory_map: &'a [MemoryMapEntry],
}

// An iterator that returns only memory map entries with the given type
impl<'a> MemoryMap<'a> {
    pub fn new(bootloader_info: &BootloaderInfo) -> Self {
        let ptr = bootloader_info.memory_map_addr as *const MemoryMapEntry;
        let size = bootloader_info.memory_map_count as usize;
        unsafe {
            // Create slice from ptr(0x500, start of memory map) and size
            let slice = slice::from_raw_parts(ptr, size);
            MemoryMap {
                memory_map: slice,
            }
        }
    }

    pub fn amount_of_free_areas(&self) -> usize {
        let mut count = 0;
        for area in self.memory_map.iter() {
            if area.get_region_type() as u32 == MemoryAreaType::Free as u32 {
                count += 1;
            }
        }
        return count;
    }
}