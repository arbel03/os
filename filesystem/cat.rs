#![feature(alloc)]
#![feature(start)]
#![no_std]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate std;

use std::io;

#[start]
#[no_mangle]
pub fn start(argc: isize, args: *const *const u8) -> isize {
    test_arg();
    // use core::str;
    // println!("argc: {}, args: {:?}", argc, args);
    // println!("*args: {:?}", unsafe { *args });

    // let file_name = unsafe {
    //     let arg_ptr: *const u8 = *args as *const u8;
    //     std::args::terminated_string(arg_ptr) 
    // };

    // println!("Filename: {}", file_name);
    // let fd = std::syscalls::open(file_name);
    // if fd != 0xffffffff {
    //     let mut vector = vec![0u8;512];
    //     std::syscalls::read(fd, &mut vector);
    //     print!("{}", unsafe { str::from_utf8_unchecked(&vector) });
    // } else {
    //     println!("Error opening file.");
    // }
    0
}

pub fn test_arg() {
    let argument = "Hello this string is null terminated.\x00";
    let parsed_string = unsafe { std::args::terminated_string(argument.as_ptr()) };
}