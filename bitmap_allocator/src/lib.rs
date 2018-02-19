#![feature(const_fn)]
#![feature(alloc, allocator_api)]
#![no_std]

#[cfg(test)]
#[macro_use] // for print
extern crate std;
extern crate alloc;

#[cfg(test)]
mod test;
mod cell;

use core::mem;
use cell::CellState;
use alloc::allocator::{ Alloc, Layout, AllocErr };

pub struct BitmapAllocator {
    bitmap_start: usize,
    block_count: usize,
}

impl BitmapAllocator {
    const BLOCK_SIZE: usize = mem::size_of::<usize>()*4;

    // total_size = block_count * (cell_size + BLOCK_SIZE)
    pub const fn new(start: usize, size: usize) -> Self {
        BitmapAllocator {
            bitmap_start: start,
            block_count: size / (mem::size_of::<CellState>() + BitmapAllocator::BLOCK_SIZE),
        }
    }

    pub fn init(&mut self) {
        for index in 0..self.block_count {
            *self.get_cell(index) = CellState::Free;
        }
    }

    fn get_cell(&self, index: usize) -> &mut CellState {
        if index >= self.block_count {
            panic!("Tried to access bitmap cell outside of bounds.");
        }
        let bitmap_start = self.bitmap_start as *mut CellState;
        let cell = unsafe { &mut *bitmap_start.offset(index as isize) };
        return cell;
    }

    fn get_data_start(&self) -> usize {
        return self.block_count * mem::size_of::<CellState>() + self.bitmap_start as usize;
    }
}

unsafe impl Alloc for BitmapAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let requested_size = layout.size() + layout.align();

        let mut cell_index: Option<usize> = None;
        let mut continuous_count = 0;
        for index in 0..self.block_count {
            let current = self.get_cell(index);
            if *current == CellState::Free { 
                continuous_count += 1;
                if cell_index == None {
                    cell_index = Some(index);
                }
            } else {
                continuous_count = 0;
                cell_index = None;
            }

            if (continuous_count * BitmapAllocator::BLOCK_SIZE) >= requested_size {
                break;                
            }
        }
        
        if let Some(cell_index) = cell_index {
            if continuous_count * BitmapAllocator::BLOCK_SIZE >= requested_size {
                let block_address = cell_index * BitmapAllocator::BLOCK_SIZE + self.get_data_start();
                let alloc_start = align_up(block_address, layout.align());

                *self.get_cell(cell_index) = CellState::Boundary;
                for index in 1..continuous_count {
                    *self.get_cell(cell_index+index) = CellState::Allocated;
                }

                return Ok(alloc_start as *mut u8);
            }
        }

        return Err(AllocErr::Exhausted {
            request: layout,
        });
    }

    #[allow(unused_variables)]
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let mut starting_block = (ptr as usize-self.get_data_start()) / BitmapAllocator::BLOCK_SIZE;
        while *self.get_cell(starting_block) != CellState::Boundary {
            starting_block -= 1;
        }

        let mut block = starting_block;
        loop {
            *self.get_cell(block) = CellState::Free;
            block += 1;
            let next_value = *self.get_cell(block);
            if next_value != CellState::Allocated {
                break;
            }
        }
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