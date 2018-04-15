#![feature(global_allocator)]
#![feature(lang_items)]
#![feature(asm)]
#![feature(start)]
#![no_std]

extern crate rlibc;
extern crate bitmap_allocator;

mod syscall;
pub mod fs;
pub mod io;
 
// Setting up heap
use bitmap_allocator::BitmapAllocator;

const HEAP_SIZE: usize = 1024*5;
static mut HEAP_AREA: [u8;HEAP_SIZE] = [0u8;HEAP_SIZE];

#[global_allocator]
static mut HEAP: BitmapAllocator = BitmapAllocator::new(0, HEAP_SIZE, ::core::mem::size_of::<usize>());

#[no_mangle]
pub unsafe fn _start() {
    io::printf("Process started.\n");
    HEAP.set_bitmap_start(&HEAP_AREA as *const u8 as usize);
    HEAP.init();

    extern "Rust" {
        fn main(argc: usize, args: *const str);
    }
    main(0, "args");
    exit();
    loop {};
}

pub unsafe fn print_heap_state() {
    use bitmap_allocator::CellState;
    for i in 0..HEAP.get_block_count() {
        let cell = HEAP.get_cell(i);
        let desc = match *cell {
            CellState::Free => "_",
            CellState::Allocated => "-",
            CellState::Boundary => "<",
        };
        io::printf(desc);
    }
    io::printf("\n");
}

#[start]
#[no_mangle]
pub unsafe fn start(_argc: isize, _args: *const *const u8) -> isize {
    0
}

pub fn exit() {
    io::printf("Process quit.\n");
}

#[lang = "eh_personality"] 
extern fn eh_personality() {

}

#[no_mangle]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(_fmt: core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
    loop {}
}