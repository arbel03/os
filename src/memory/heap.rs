use alloc::heap::{ Alloc, AllocErr, Layout };
use core::sync::atomic::{ AtomicUsize, Ordering };

pub struct BumpAllocator {
    end: usize,
    current: AtomicUsize,
}

impl BumpAllocator {
    pub const fn new(start: usize, size: usize) -> Self {
        BumpAllocator { end: start+size, current:AtomicUsize::new(start) }
    }
}

unsafe impl <'a> Alloc for &'a BumpAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        loop {
            // println!("Reqeusting layout: {:?}", layout);
            let current = self.current.load(Ordering::Relaxed);
            let alloc_start = align_up(current, layout.align());
            let alloc_end = alloc_start + layout.size();

            if alloc_end <= self.end {
                let new_current = self.current.compare_and_swap(current, alloc_end, Ordering::Relaxed);
                if new_current == current {
                    return Ok(alloc_start as *mut u8);
                }
            } else {
                return Err(AllocErr::Exhausted{ request: layout });
            }
        }
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        // Do nothing, allow memory leaks
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
