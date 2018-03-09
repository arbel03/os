#![no_std]
#![feature(lang_items)] 
#![feature(start)]

#[start]
fn main(argc: isize, args: *const *const u8) -> isize {
    //println!("Hello, world!");
    return 1;
}

#[lang = "eh_personality"] extern fn eh_personality() { }

#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    // println!("PANIC in {} at line {}:", file, line);
    // println!("{}", fmt);
    loop {}
}