#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(const_unique_new)]
#![feature(compiler_builtins_lib)]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![no_std]

extern crate rlibc;
extern crate spin;

#[macro_use]
mod vga_buffer;
mod dtables;
mod drivers;
mod interrupts;
mod memory;

#[no_mangle]
pub extern fn kmain(free_address_start: usize) {
    vga_buffer::clear_screen();
    println!("free_address_start: 0x{:x}", free_address_start);

    interrupts::init();
    memory::init();
}

// #[no_mangle] pub extern "C" fn __udivdi3() { loop {} }
// #[no_mangle] pub extern "C" fn __umoddi3() { loop {} }
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! { loop{} }
