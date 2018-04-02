use super::segmentation::*;
use task::state::TaskStateSegment;
use dtables::*;

pub type SegmentDescriptorEntry = u64;

pub type SegmentDescriptorTable = DescriptorTable<SegmentDescriptorEntry>;

pub trait Gdt {
    fn init(&mut self);
    fn set_tss(&mut self, tss: &TaskStateSegment);
    fn set_ldt(&mut self, ldt: &SegmentDescriptorTable);
    fn get_selector(&self, segment_type: SegmentType, privilege_level: usize) -> u32;
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
        // GDT entry should look like this:
        // 00000000 00000000 00000000 01100111
        // 00000000 10000000 10001001 00000000
        let tss_size = ::core::mem::size_of::<TaskStateSegment>() as u32;
        self.insert(SegmentType::TssDescriptor as usize, SegmentDescriptor::new(tss as *const _ as u32, tss_size, 0b10001001, 0b1000));
    }

    fn set_ldt(&mut self, ldt: &SegmentDescriptorTable) {
        // GDT entry should look like this:
        // 00000000 00000000 00000000 00001111
        // 00000000 10000000 10000010 00000000 
        let table_descriptor = TableDescriptor::new(ldt);
        self.insert(SegmentType::LdtDescriptor as usize, SegmentDescriptor::new(table_descriptor.ptr, table_descriptor.limit as u32 + 1, 0b11100010 , 0b1000));
    }
    fn set_descriptor(&mut self, segment_type: SegmentType, descriptor: SegmentDescriptor) {
        self.insert(segment_type as usize, descriptor);
    }

    fn get_selector(&self, segment_type: SegmentType, privilege_level: usize) -> u32 {
        SegmentSelector::new(segment_type as usize, TableType::GDT, privilege_level)
    }

    unsafe fn load(&self) {
        let pointer = TableDescriptor::new(self);
        // Loading only GDT
        lgdt(&pointer);
    }
}