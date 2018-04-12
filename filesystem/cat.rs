#![no_std]
#![no_main]

extern crate std;

#[no_mangle]
pub fn main() {
    let fd = std::fs::open("bin/elffile");
    loop {};
}