#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(const_unique_new)]
#![no_std]
#![feature(compiler_builtins_lib)]
#![feature(asm)]
#![feature(abi_x86_interrupt)]

extern crate rlibc;
extern crate spin;
extern crate compiler_builtins;

#[macro_use]
mod vga_buffer;
pub mod dtables;
mod interrupts;
mod memory;

#[no_mangle]
pub extern fn kmain(free_address_start: usize) {
    vga_buffer::clear_screen();
    println!("free_address_start: 0x{:x}", free_address_start);

    interrupts::init();
    memory::init();
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! { loop{} }
