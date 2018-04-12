#![feature(lang_items)]
#![feature(asm)]
#![feature(start)]
#![feature(global_allocator)]
#![no_std]

extern crate bitmap_allocator;
extern crate rlibc;

mod syscall;
pub mod fs;
pub mod io;

const HEAP_SIZE: usize = 1024*50;
static HEAP_AREA: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];

use bitmap_allocator::BitmapAllocator;
#[global_allocator]
static mut HEAP: BitmapAllocator = BitmapAllocator::new(0x0, HEAP_SIZE, core::mem::size_of::<usize>()*4);

#[start]
#[no_mangle]
pub unsafe fn _start(_argc: isize, _args: *const *const u8) -> isize {
    HEAP.set_bitmap_start(&HEAP_AREA as *const _ as usize);
    HEAP.init();
    extern {
        fn main();
    }
    main(); 
    0
}

#[lang = "eh_personality"] 
extern fn eh_personality() {

}

#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(_fmt: core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    loop {}
}