use alloc::boxed::Box;
use alloc::vec::Vec;
use core::mem;

#[repr(C, packed)]
#[derive(Debug)]
pub struct TableDescriptor { 
    pub limit: u16, // Size of the table
    pub ptr: u32, // pointer
}

impl TableDescriptor {
    // This function creates a new TableDescriptor from a slice.
    pub fn new<T: Default>(structure: &DescriptorTable<T>) -> Self {
        if let Some(slice) = structure.as_ref() {
            let size = mem::size_of::<T>() * slice.len() - 1;

            return TableDescriptor {
                limit: size as u16,
                ptr: slice.as_ptr() as *const _ as u32,
            };
        }
        TableDescriptor::empty()
    }

    pub fn empty() -> Self {
        TableDescriptor {
            limit: 0,
            ptr: 0,
        }
    }
}

pub trait Encodable<T> {
    fn encode(&self) -> T;
}

pub struct DescriptorTable<T: Default> {
    table: Option<Box<[T]>>
}

impl <T: Default> DescriptorTable<T> {
    pub const fn new() -> Self {
        DescriptorTable { 
            table: None 
        }
    }

    pub fn insert<S: Encodable<T>>(&mut self, index: usize, element: S) {
        if index > self.table.as_ref().unwrap().len() {
            panic!("Index out of table range.");
        }
        self.table.as_mut().unwrap()[index] = element.encode();
    }

    pub fn init_with_length(&mut self, length: usize) {
        let mut vec = Vec::with_capacity(length);
        for _ in 0..length {
            vec.push(T::default());
        }
        self.table = Some(vec.into_boxed_slice())
    }

    pub fn as_ref(&self) -> Option<&[T]> {
        if let Some(boxed_slice) = self.table.as_ref() {
            return Some(boxed_slice);
        }
        None
    }
}

#[inline(always)]
pub unsafe fn lgdt(gdt: &TableDescriptor) {
    asm!("lgdt [$0]" :: "r"(gdt) : "memory" : "intel", "volatile");
}

#[inline(always)]
pub unsafe fn lidt(idt: &TableDescriptor) {
    asm!("lidt [$0]" :: "r"(idt) : "memory" : "intel", "volatile");
}
