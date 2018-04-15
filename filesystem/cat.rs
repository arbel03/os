#![feature(alloc)]
#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;
extern crate std;

#[no_mangle]
pub fn main(argc: usize, args: *const str) {
    use alloc::string::ToString;
    let string = "This string is in the heap.".to_string();
    std::io::printf(&string);
    // std::io::printf("\n");
    // print_heap();
}

fn print_heap() {
    unsafe {
        std::print_heap_state();
    }
}