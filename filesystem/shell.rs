#![feature(asm)]
#![feature(start)]
#![no_std]

extern crate std;

#[no_mangle]
pub fn _start() {
    main(0, 0 as *const *const u8);
    loop {};
}

#[start]
fn main(argc: isize, args: *const *const u8) -> isize {
    use std::io::printf;

    printf("Hello from process!\n");
    let fd = std::fs::open("MyFile");
    
    return 0;
}