#![no_std]
#![no_main]

extern crate std;

#[no_mangle]
pub fn main() {
    std::io::printf("Welcome to my OS!\n");
}