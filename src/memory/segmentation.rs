use dtables::Encodable;

// Segment Selector

#[repr(u8)]
pub enum AccessFlags {
    Present = 1 << 7,
    PL0 = 0,
    PL3 = 3 << 5,
    Executable = 1 << 3,
    Direction = 1 << 2,
    ReadWrite = 1 << 1,
    Accessed = 1,
    AlwaysOne = 1 << 4,
}

#[repr(u8)]
pub enum Flags {
    Granularity = 1 << 3,
    Size = 1 << 2,
}

pub enum TableType { 
    GDT = 0, 
    LDT = 1,
}

pub struct SegmentSelector;

impl SegmentSelector {
    pub const fn new(index: usize, table: TableType, protection_level: usize) -> u16 {
        ((index << 3) | ((table as usize) << 2) | (protection_level)) as u16
    }
}

// Segment Descriptor

#[derive(Debug, Clone)]
pub struct SegmentDescriptor {
    pub base: u32,
    pub limit: u32,
    pub access_byte: u8,
    pub flags: u8
}

impl SegmentDescriptor {
    pub fn new(base: u32, limit: u32, access_byte: u8, flags: u8) -> SegmentDescriptor {
        SegmentDescriptor { base, limit, access_byte, flags }
    }
}

use super::gdt::SegmentDescriptorEntry;
impl Encodable<SegmentDescriptorEntry> for SegmentDescriptor {
    fn encode(&self) -> SegmentDescriptorEntry {
        let mut descriptor_high: u32;
        // Create the high 32 bit segment
        descriptor_high  =  (self.base & 0x00FF0000) >> 16;
        descriptor_high |= (self.access_byte as u32) << 8;
        descriptor_high |= self.limit & 0x000F0000;
        descriptor_high |=  (self.flags as u32) << 20;
        descriptor_high |= self.base & 0xFF000000;        

        let mut descriptor_low: u32 = 0;
        // Create the low 32 bit segment
        descriptor_low |= self.base << 16;                 // set base bits 15:0
        descriptor_low |= self.limit & 0xFFFF;               // set limit bits 15:0

        let mut descriptor: u64 = (descriptor_high as u64) << 32;
        descriptor |= descriptor_low as u64;
        return descriptor;
    }
}