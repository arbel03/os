#![feature(global_allocator)]
#![feature(lang_items)]
#![feature(asm)]
#![feature(start)]
#![feature(alloc)]
#![no_std]

extern crate rlibc;
extern crate bitmap_allocator;
extern crate alloc;

#[macro_use]
pub mod io;
pub mod args;
pub mod syscalls;
 
// Setting up heap
use bitmap_allocator::BitmapAllocator;

pub const HEAP_SIZE: usize = 1024*20;
pub static mut HEAP_AREA: [u8;HEAP_SIZE] = [0u8;HEAP_SIZE];

#[global_allocator]
static mut HEAP: BitmapAllocator = BitmapAllocator::new(0, HEAP_SIZE, ::core::mem::size_of::<usize>());

#[no_mangle]
#[start]
pub unsafe extern "C" fn _start(argc: isize, argv: *const *const u8) -> isize {
    HEAP.set_bitmap_start(&HEAP_AREA as *const u8 as usize);
    HEAP.init();

    extern "Rust" {
        fn main(argc: usize, args: *const *const u8);
    }

    main(argc as usize, argv);
    exit();
    0
}

unsafe fn exit() {
    asm!("int 0x82" :::: "intel");
}

#[lang = "eh_personality"] 
extern fn eh_personality() {

}

#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(_fmt: core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    unsafe { exit(); }
    loop {};
}