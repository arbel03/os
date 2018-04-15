#![feature(alloc)]
#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;
extern crate std;

#[no_mangle]
pub fn main(argc: usize, args: *const str) {
    std::io::printf("Hello From Process!\n");
    let fd = std::fs::open("bin/elffile");
    std::io::printf("args: ");
    std::io::printf(unsafe { &*args });
    std::io::printf("\n");
    let mut string = alloc::String::new();
    // string.push_str("This string is stored in the heap.");
    std::io::printf(&string);
    print_heap();
}

fn print_heap() {
    
    use alloc::string::ToString;
    let hi = "Hello world";
    let heaped_hi = hi.to_string();
    unsafe {
        std::print_heap_state();
    }
}