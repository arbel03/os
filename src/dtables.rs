use core::mem;

#[repr(C, packed)]
#[derive(Debug)]
pub struct TableDescriptor { 
    pub limit: u16, // Size of the table
    pub ptr: u32, // pointer
}

impl TableDescriptor {
    // This function creates a new TableDescriptor from a slice.
    pub fn new<T>(structure: &[T]) -> Self {
        let size = mem::size_of_val(structure) - 1;

        TableDescriptor {
            limit: size as u16,
            ptr: structure.as_ptr() as *const _ as u32,
        }
    }
}

#[inline(always)]
pub unsafe fn lgdt(gdt: &TableDescriptor) {
    asm!("lgdt [$0]" :: "r"(gdt) : "memory" : "intel");
}

#[inline(always)]
pub unsafe fn lidt(idt: &TableDescriptor) {
    asm!("lidt [$0]" :: "r"(idt) : "memory" : "intel");
}
