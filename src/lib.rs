#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(const_unique_new)]
#![no_std]

extern crate rlibc;
mod vga_buffer;
mod gdt;

#[no_mangle]
pub extern fn rust_main() {

    vga_buffer::print_something();

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
