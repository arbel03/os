#![feature(alloc)]
#![feature(start)]
#![no_std]

#[macro_use]
extern crate alloc;
extern crate std;

use std::io;

#[start]
#[no_mangle]
pub fn start(argc: isize, args: *const *const u8) -> isize {
    0
}