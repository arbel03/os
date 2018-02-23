use alloc::boxed::Box;
use alloc::vec::Vec;
use dtables::{ TableDescriptor, lgdt };

fn encode_entry(entry: SegmentDescriptor) -> DescriptorEntry {
    let mut descriptor_high: u32;
    // Create the high 32 bit segment
    descriptor_high  =  (entry.base & 0x00FF0000) >> 16;
    descriptor_high |= (entry.access_byte as u32) << 8;
    descriptor_high |= entry.limit & 0x000F0000;
    descriptor_high |=  (entry.flags as u32) << 20;
    descriptor_high |= entry.base & 0xFF000000;        

    let mut descriptor_low: u32 = 0;
    // Create the low 32 bit segment
    descriptor_low |= entry.base << 16;                 // set base bits 15:0
    descriptor_low |= entry.limit & 0xFFFF;               // set limit bits 15:0

    let mut descriptor: u64 = (descriptor_high as u64) << 32;
    descriptor |= descriptor_low as u64;
    return descriptor;
}

pub type DescriptorEntry = u64;

pub struct DescriptorTable(Box<[DescriptorEntry]>);

impl DescriptorTable {
    pub fn new() -> Self {
        DescriptorTable(Box::new([]))
    }

    pub fn set_entries(&mut self, descriptors: Vec<SegmentDescriptor>) {
        let mut gdt: Vec<u64> = Vec::with_capacity(descriptors.len()); 
        for descriptor in descriptors {
            gdt.push(encode_entry(descriptor));
        }

        self.0 = gdt.into_boxed_slice();
    }

    pub unsafe fn load(&self) {
        use core::ops::Deref;

        let gdt_entries = self.0.deref();
        // Loading gdt
        let gdtr = TableDescriptor::new(gdt_entries);
        println!("Gdtr: {:?}", gdtr);
        lgdt(&gdtr);
    }
}

#[repr(packed, C)]
#[derive(Clone, Copy, Debug)]
pub struct SegmentDescriptor {
    pub base: u32,
    pub limit: u32,
    pub access_byte: u8,
    pub flags: u8
}

impl SegmentDescriptor {
    pub const NULL: SegmentDescriptor = SegmentDescriptor { 
        base: 0, 
        limit: 0, 
        access_byte: 0, 
        flags: 0 
    };

    pub fn new(base: u32, limit: u32, access_byte: u8, flags: u8) -> SegmentDescriptor {
        SegmentDescriptor { base, limit, access_byte, flags }
    }
}