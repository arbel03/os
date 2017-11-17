#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(const_unique_new)]
#![no_std]
#![feature(compiler_builtins_lib)]

extern crate rlibc;
extern crate spin;
extern crate compiler_builtins;

#[macro_use]
mod vga_buffer;
mod gdt;

#[no_mangle]
pub extern fn rust_main() {

    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    println!("{} + {} = {}", 1, 2, 1+2);

    match gdt::add_descriptor() {
        Ok(descriptor) => println!("{:?}", descriptor),
        Err(err) => println!("{}", err),
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! { loop{} }
