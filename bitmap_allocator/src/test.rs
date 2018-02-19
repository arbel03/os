use super::*;
use std::boxed::Box;
use std::prelude::v1::*;
use std::mem::{size_of, align_of};
use alloc::allocator::Layout;

fn get_allocator() -> BitmapAllocator {
    const HEAP_SIZE: usize = 1000;
    let heap_space = Box::into_raw(Box::new([0u8; HEAP_SIZE]));
    let mut allocator = BitmapAllocator::new(heap_space as usize, HEAP_SIZE);
    allocator.init();
    return allocator;
}

fn print_bitmap(allocator: &BitmapAllocator) {
    println!("Printing bitmap:");
    let bitmap_size = allocator.block_count;
    for index in 0..bitmap_size {
        let block = allocator.get_cell(index).clone();
        let block_string = match block {
            CellState::Free => "_",
            CellState::Boundary => "*",
            CellState::Allocated => ">",
        };
        print!("{} ", block_string);
        if (index+1) % 10 == 0 {
            print!("\n");
        }
    }
    print!("\n");
}

#[test]
fn test_single_allocation() {
    use alloc::allocator::Alloc;
    let mut heap = get_allocator();
    let size = size_of::<usize>();
    let layout = Layout::from_size_align(size, align_of::<usize>());
    let addr = unsafe { Alloc::alloc(&mut heap, layout.clone().unwrap()) };
    assert!(addr.is_ok());
    let addr = addr.unwrap() as usize;
    println!("");
    println!("allocating layout: {:?}", layout.clone().unwrap());
    println!("allocated at: {}", addr);
    print_bitmap(&heap);
    println!("deallocating at: {}", addr);
    unsafe { Alloc::dealloc(&mut heap, addr as *mut u8, layout.clone().unwrap()) };
    print_bitmap(&heap);
    assert!(addr == align_up(heap.get_data_start(), align_of::<usize>()));
}

#[test]
fn test_multiple_allocation() {
    use alloc::allocator::Alloc;
    let mut heap = get_allocator();
    print_bitmap(&heap);
    let size = size_of::<usize>()*6;
    let mut addresses: Vec<usize> = Vec::new();
    let mut layouts: Vec<Layout> = Vec::new();
    for _ in 0..10 {
        let layout = Layout::from_size_align(size, align_of::<usize>()).unwrap();
        let addr = unsafe { Alloc::alloc(&mut heap, layout.clone()) };
        assert!(addr.is_ok());
        let addr = addr.unwrap() as usize;
        println!("allocated at {}, layout: {:?}", addr, layout.clone());
        addresses.push(addr);
        layouts.push(layout);
        print_bitmap(&heap);
    }

    for i in 0..addresses.len() {
        unsafe { Alloc::dealloc(&mut heap, addresses[i] as *mut u8, layouts[i].clone()) };
    }

    let mut all_free = true;
    for i in 0..heap.block_count {
        if *heap.get_cell(i) != CellState::Free {
            all_free = false;
        }
    }
    print_bitmap(&heap);
    assert!(all_free);
}