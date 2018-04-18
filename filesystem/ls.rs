#![feature(alloc)]
#![feature(start)]
#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;
extern crate std;

use std::io;

#[no_mangle]
pub fn main(argc: isize, args: *const *const u8) {
}