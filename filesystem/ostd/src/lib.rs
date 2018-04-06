#![feature(lang_items)]
#![feature(asm)]
#![feature(start)]
#![no_std]

mod io;

mod fs;
pub use self::fs::*;

#[lang = "eh_personality"] extern fn eh_personality() { }

#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! { loop {} }