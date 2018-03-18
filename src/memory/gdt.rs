use super::segmentation::*;
use task::state::TaskStateSegment;
use dtables::*;

pub type SegmentDescriptorEntry = u64;

pub type SegmentDescriptorTable = DescriptorTable<SegmentDescriptorEntry>;

pub trait Gdt {
    fn init(&mut self);
    fn set_tss(&mut self, tss: &TaskStateSegment);
    fn set_ldt(&mut self, ldt: &SegmentDescriptorTable);
    fn get_selector(&self, segment_type: SegmentType, privilege_level: usize) -> SegmentSelector;
    fn set_descriptor(&mut self, segment_type: SegmentType, descriptor: SegmentDescriptor);
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
        let tss_size = ::core::mem::size_of::<TaskStateSegment>() as u32;
        self.insert(SegmentType::TssDescriptor as usize, SegmentDescriptor::new(tss as *const _ as u32, tss_size, 0xE9, 0x0));
    }

    fn set_ldt(&mut self, ldt: &SegmentDescriptorTable) {
        let table_descriptor = TableDescriptor::new(ldt);
        // This descriptor should change in the future
        self.insert(SegmentType::LdtDescriptor as usize, SegmentDescriptor::new(table_descriptor.ptr, table_descriptor.limit as u32, 0b0000010, 0x0000));
    }

    fn set_descriptor(&mut self, segment_type: SegmentType, descriptor: SegmentDescriptor) {
        self.insert(segment_type as usize, descriptor);
    }

    fn get_selector(&self, segment_type: SegmentType, privilege_level: usize) -> SegmentSelector {
        SegmentSelector::new(segment_type as usize, TableType::GDT, privilege_level)
    }

    unsafe fn load(&self) {
        let pointer = TableDescriptor::new(self);
        // Loading only GDT
        lgdt(&pointer);
    }
}