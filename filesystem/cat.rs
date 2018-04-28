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
    use core::str;

    let args = &unsafe { std::args::get_args(argc, argv) };
    if argc != 2 {
        let file_name = args[0];
        println!("Usage:\n\t{} <file_name>", file_name);
        return;
    }

    let file_name = args[1];
    let fd = std::syscalls::open(file_name);
    if fd != 0xffffffff {
        let file_size = std::syscalls::filesz(fd);
        let mut vector = vec![0u8;file_size];
        std::syscalls::read(fd, &mut vector);
        println!("{}", unsafe { str::from_utf8_unchecked(&vector) });
    } else {
        println!("Error opening file.");
    }
}