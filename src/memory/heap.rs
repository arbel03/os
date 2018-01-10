use alloc::heap::{Alloc, AllocErr, Layout};

pub struct BitmapAllocator {
    start: usize,
    end: usize,
}

impl BitmapAllocator {
    pub const fn new(start: usize, end: usize) -> Self {
        BitmapAllocator {
            start: start, 
            end: end, 
        }
    }
}

unsafe impl <'a> Alloc for &'a BitmapAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        //let size = layout.size();
        // TODO: Implement
        Err(AllocErr::Exhausted {request: layout })
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
    }
}