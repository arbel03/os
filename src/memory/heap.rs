use spin::Mutex;
use alloc::heap::{ Alloc, AllocErr, Layout };

pub struct BumpAllocator {
    end: usize,
    current: usize,
}

impl BumpAllocator {
    pub const fn new(start: usize, size: usize) -> Self {
        BumpAllocator { end: start + size, current: start }
    }
}

unsafe impl Alloc for BumpAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let current = self.current;
        let alloc_start = align_up(current, layout.align());
        let alloc_end = alloc_start + layout.size();
        
        if alloc_end <= self.end {
            self.current = alloc_end;
            // println!("Allocating at {:#x}", alloc_start);
            return Ok(alloc_start as *mut u8);
        } else {
            print!("Allocator exhausted.");
            return Err(AllocErr::Exhausted{ request: layout });
        }
    }

    #[allow(unused_variables)]
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        unimplemented!();
    }
}

/// Align downwards. Returns the greatest x with alignment `align`
/// so that x <= addr. The alignment must be a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    } else if align == 0 {
        addr
    } else {
        panic!("`align` must be a power of 2");
    }
}

/// Align upwards. Returns the smallest x with alignment `align`
/// so that x >= addr. The alignment must be a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}


// Locked bump allocator
pub struct LockedAllocator(Mutex<BumpAllocator>);

impl LockedAllocator {
    pub const fn new(start: usize, size: usize) -> Self {
        LockedAllocator(Mutex::new(BumpAllocator::new(start, size)))
    }
}

unsafe impl<'a> Alloc for &'a LockedAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        self.0.lock().alloc(layout)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        self.0.lock().dealloc(ptr, layout)
    }
}
