#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(const_unique_new)]
#![feature(compiler_builtins_lib)]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
// #![feature(alloc)]
// #![feature(allocator_api)]
// #![feature(global_allocator)]
#![no_std]

// extern crate alloc;
extern crate rlibc;
extern crate spin;

#[macro_use] // vec! macro
mod vga_buffer;
mod dtables;
mod drivers;
mod interrupts;
mod memory;

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct BootloaderInfo {
    memory_map_count: u32,
    memory_map_addr: u32,
    kernel_start: u32,
    kernel_end: u32,
}

// use memory::heap::BitmapAllocator;
//#[global_allocator]
//static HEAP: BitmapAllocator = BitmapAllocator::new(0, 100);

#[no_mangle]
pub extern fn kmain(bootloader_info: &BootloaderInfo) {
    vga_buffer::clear_screen();
    println!("Bootloader info: {:?}", bootloader_info);

    memory::init(bootloader_info);
    drivers::configure();
    interrupts::init();
}


#[lang = "eh_personality"] extern fn eh_personality() { }

#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    print!("\n\n\n");
    println!("PANIC in {} at line {}:", file, line);
    println!("{}", fmt);
    loop{}
}