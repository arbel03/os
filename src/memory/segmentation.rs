use alloc::boxed::Box;
use alloc::vec::Vec;
use dtables::{ TableDescriptor, lgdt };

#[repr(usize)]
pub enum SegmentTable {
    GDT = 0,
    LDT = 1,
}

pub struct SegmentSelector(pub u16);

impl SegmentSelector {
    pub const fn new(index: usize, table: SegmentTable, protection_level: usize) -> Self {
        SegmentSelector((index << 3 | (table as usize) << 2 | protection_level) as u16)
    }
}

#[inline(always)]
pub unsafe fn load_cs(segment: SegmentSelector) {
    asm!("
    mov ax, $0
    jmp ax:.flush_cs
.flush_cs:
    " :: "m"(segment.0) : "ax" : "intel","volatile");
}

#[inline(always)]
pub unsafe fn load_ds(segment: SegmentSelector) {
    asm!("
    mov ax, $0
    mov ds, ax
    " :: "m"(segment.0) : "ax" : "intel","volatile");
}

#[inline(always)]
pub unsafe fn load_ss(segment: SegmentSelector) {
 asm!("
    mov ax, $0
    mov ss, ax
    " :: "m"(segment.0) : "ax" : "intel","volatile");
}

pub type DescriptorEntry = u64;

pub struct DescriptorTable(Option<Box<[DescriptorEntry]>>);

impl DescriptorTable {
    pub const fn new() -> Self {
        DescriptorTable(None)
    }

    pub fn set_entries(&mut self, descriptors: Vec<SegmentDescriptor>) {
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

        let mut gdt: Vec<u64> = Vec::with_capacity(descriptors.len()); 
        for descriptor in descriptors {
            gdt.push(encode_entry(descriptor));
        }

        self.0 = Some(gdt.into_boxed_slice());
    }

    pub unsafe fn load(&self) {
        use core::ops::Deref;

        let gdt_entries = self.0.as_ref().unwrap().deref();
        // Loading gdt
        let gdtr = TableDescriptor::new(gdt_entries);
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