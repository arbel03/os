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
    println!("argc: {:?}", argc);
    println!("argv: {:?}", argv);

    let file_name = unsafe {
        let arg_ptr: *const u8 = *argv as *const u8;
        std::args::terminated_string(arg_ptr) 
    };

    println!("Filename: {}", file_name);
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