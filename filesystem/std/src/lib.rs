#![feature(global_allocator)]
#![feature(lang_items)]
#![feature(asm)]
#![feature(start)]
#![no_std]

extern crate rlibc;
extern crate bitmap_allocator;

#[macro_use]
pub mod io;
pub mod args;
pub mod syscalls;
 
// Setting up heap
use bitmap_allocator::BitmapAllocator;

const HEAP_SIZE: usize = 1024*5;
static mut HEAP_AREA: [u8;HEAP_SIZE] = [0u8;HEAP_SIZE];

#[global_allocator]
static mut HEAP: BitmapAllocator = BitmapAllocator::new(0, HEAP_SIZE, ::core::mem::size_of::<usize>());

#[no_mangle]
pub unsafe fn _start() {
    HEAP.set_bitmap_start(&HEAP_AREA as *const u8 as usize);
    HEAP.init();

    extern "Rust" {
        fn start(argc: isize, args: *const *const u8) -> isize;
    }
    let file_name = "files/elffile\x00";
    let arg1_ptr = file_name.as_ptr();
    start(1, &arg1_ptr as *const *const u8);
    exit();
}

pub fn exit() {
    print!("Process quit.\n");
    loop {};
}

#[lang = "eh_personality"] 
extern fn eh_personality() {

}

#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(_fmt: core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    loop {}
}