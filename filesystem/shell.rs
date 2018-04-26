#![feature(alloc)]
#![feature(start)]
#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate std;

use std::io;

#[no_mangle]
pub fn main(argc: usize, argv: *const *const u8) {
    let args = &unsafe { std::args::get_args(argc, argv) };
    loop {
        print!("{} $ ", args[0]);
        let input = io::read_string();
        if input == "quit" {
            break;
        } else if input == "print heap" {
            unsafe { std::print_heap(); }
        }
    }
}