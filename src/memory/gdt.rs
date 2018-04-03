use super::segmentation::*;
use task::state::TaskStateSegment;
use dtables::*;

pub enum DescriptorType {
    NullDescriptor = 0,
    KernelCode = 1,
    KernelData = 2,
    TssDescriptor = 3,
    LdtDescriptor = 4,
    UserCode = 5,
    UserData = 6,
}

pub type SegmentDescriptorEntry = u64;

pub type SegmentDescriptorTable = DescriptorTable<SegmentDescriptorEntry>;

pub trait Gdt {
    fn init(&mut self);
    fn set_tss(&mut self, tss: &TaskStateSegment);
    fn set_ldt(&mut self, ldt: &SegmentDescriptorTable);
    fn get_selector(&self, segment_type: DescriptorType, privilege_level: usize) -> u32;
    fn set_descriptor(&mut self, segment_type: DescriptorType, descriptor: SegmentDescriptor);
    unsafe fn load(&self);
}

// Null
// Code Segment- PL0 
// Data Segment- PL0
// TSS
// LDT
// Code Segment- PL3
// Data Segment- PL3
impl Gdt for SegmentDescriptorTable {
    fn init(&mut self) {
        self.init_with_length(7);
    }

    fn set_tss(&mut self, tss: &TaskStateSegment) {
        // GDT entry should look like this:
        // 00000000 00000000 00000000 01100111
        // 00000000 10000000 10001001 00000000
        let base = tss as *const _ as u32;
        let size = ::core::mem::size_of::<TaskStateSegment>() as u32;
        let limit = base + size;
        self.insert(DescriptorType::TssDescriptor as usize, SegmentDescriptor::new(base, limit, 0b10001001, 0b0100));
    }

    fn set_ldt(&mut self, ldt: &SegmentDescriptorTable) {
        // GDT entry should look like this:
        // 00000000 00000000 00000000 00001111
        // 00000000 10000000 10000010 00000000 
        let table_descriptor = TableDescriptor::new(ldt);
        let base = table_descriptor.ptr;
        let size = table_descriptor.limit as u32 + 1;
        let limit = base + size;
        self.insert(DescriptorType::LdtDescriptor as usize, SegmentDescriptor::new(base, limit, 0b11100010 , 0b0100));
    }
    fn set_descriptor(&mut self, segment_type: DescriptorType, descriptor: SegmentDescriptor) {
        self.insert(segment_type as usize, descriptor);
    }

    fn get_selector(&self, segment_type: DescriptorType, privilege_level: usize) -> u32 {
        SegmentSelector::new(segment_type as usize, TableType::GDT, privilege_level)
    }

    unsafe fn load(&self) {
        let pointer = TableDescriptor::new(self);
        // Loading only GDT
        lgdt(&pointer);
    }
}