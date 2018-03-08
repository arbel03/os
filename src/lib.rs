#![feature(lang_items)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(const_unique_new)]
#![feature(compiler_builtins_lib)]
#![feature(asm)]
#![feature(abi_x86_interrupt)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(global_allocator)]
#![feature(core_intrinsics)]
#![feature(use_extern_macros)]
#![feature(naked_functions)]
#![no_std]
#![allow(safe_packed_borrows)]

#[macro_use]
extern crate alloc;
extern crate rlibc;
extern crate spin;
extern crate bitmap_allocator;

#[macro_use] // vec! macro
pub mod vga_buffer;
mod dtables;
mod drivers;
mod interrupts;
mod memory;
mod filesystem;
mod syscall;

pub use interrupts::syscall::syscall_handler_inner;

#[derive(Debug, Copy, Clone)]
#[repr(C, packed)]
pub struct BootloaderInfo {
    memory_map_count: u32,
    memory_map_addr: u32,
    kernel_start: u32,
    kernel_end: u32,
}

use memory::heap::Heap;
use bitmap_allocator::BitmapAllocator;
#[global_allocator]
static HEAP: Heap = Heap::new(BitmapAllocator::new(0x1000000, 1024*100)); // 100 KB

#[no_mangle]
pub extern fn kmain(bootloader_info: &BootloaderInfo) {
    vga_buffer::clear_screen();
    
    memory::init(bootloader_info); 
    drivers::configure();
    interrupts::init();

    filesystem::init();

    // Test filesystem syscalls
    use syscall::fs::{ open, read, seek };
    let mut read_buffer = vec![0u8;20];
    unsafe {
        let fd = open("TESTFILETXT");
        seek(fd, 540);
        read(fd, &mut read_buffer);
    }
    println!("Read Buffer: {}", core::str::from_utf8(&read_buffer).unwrap());

    loop {};
}

#[lang = "eh_personality"] extern fn eh_personality() { }

#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("PANIC in {} at line {}:", file, line);
    println!("{}", fmt);
    loop {}
}