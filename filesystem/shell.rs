#![no_std]
#![no_main]

extern crate std;

#[no_mangle]
pub fn main() {
    use std::io::printf;
    printf("Welcome to my OS!\n");
    loop {};
}