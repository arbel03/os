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
    if fd == 0xffffffff {
        println!("Error opening file \"{}\"", file_name);
    } else if fd == 0xfffffffe {
        println!("Invalid argument - \"{}\" if a folder.", file_name);
    } else {
        println!("Printing contents of \"{}\":", file_name);
        let file_stat = std::syscalls::stat(file_name, 0);
        let mut vector = vec![0u8;file_stat.directory_size];
        std::syscalls::read(fd, &mut vector);
        println!("{}", unsafe { str::from_utf8_unchecked(&vector) });
    }
}