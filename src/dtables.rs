use core::mem;

#[repr(C, packed)]
pub struct TableDescriptor { 
    pub limit: u16, // Size of the table
    pub ptr: u32, // pointer
}

impl TableDescriptor {
    pub fn new<T>(structure: &T) -> Self {
        let size = mem::size_of::<T>() - 1;

        TableDescriptor {
            limit: size as u16,
            ptr: structure as *const _ as u32,
        }
    }
}

pub unsafe fn lgdt(gdt: &TableDescriptor) {
    asm!("lgdt [$0]" :: "r"(gdt) : "memory" : "intel");
}

#[inline(always)]
pub unsafe fn lidt(idt: &TableDescriptor) {
    asm!("lidt [$0]" :: "r"(idt) : "memory" : "intel");
}
