pub mod ldt;
pub mod idt;
use core::mem;

#[repr(C)]
pub struct TablePointer { 
    pub limit: u16, // Size of the table
    pub ptr: u32, // pointer
}

impl TablePointer {
    pub fn new<T>(table: &[T]) -> Self {
        let size = mem::size_of::<T>() * table.len() - 1;

        TablePointer {
            limit: size as u16,
            ptr: table.as_ptr() as u32,
        }
    }
}

pub unsafe fn lldt(gdt_pointer: &TablePointer) {
    asm!("lldt [$0]" :: "r"(gdt_pointer as *const _) :: "intel", "volatile");
}

pub unsafe fn lidt(idt: &TablePointer) {
    asm!("lidt ($0)" :: "r" (idt) : "memory");
}
