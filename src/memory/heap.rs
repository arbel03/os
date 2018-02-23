use bitmap_allocator::BitmapAllocator;
use spin::Mutex;
use alloc::allocator::{ Layout, Alloc, AllocErr };
use core::ops::Deref;

pub struct Heap {
    heap: Mutex<BitmapAllocator>,
}

impl Heap {
    pub const fn new(allocator: BitmapAllocator) -> Self {
        Heap {
            heap: Mutex::new(allocator),
        }
    }
}

unsafe impl <'a> Alloc for &'a Heap {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        self.heap.lock().alloc(layout)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        self.heap.lock().dealloc(ptr, layout)
    }
}

impl Deref for Heap {
    type Target = Mutex<BitmapAllocator>;

    fn deref(&self) -> &Mutex<BitmapAllocator> {
        &self.heap
    }
}