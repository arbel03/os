#![feature(asm)]
#![feature(start)]
#![no_std]

extern crate ostd;

#[start]
fn main(argc: isize, args: *const *const u8) -> isize {
    ostd::open("MyFile");
    0
}