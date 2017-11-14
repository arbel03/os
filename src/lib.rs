#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(const_unique_new)]
#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod vga_buffer;
mod gdt;

#[no_mangle]
pub extern fn rust_main() {

    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    println!("{} + {} = {}", 1, 2, 1+2);

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
