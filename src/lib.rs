#![feature(lang_items)]
#![no_std]

extern crate rlibc;

#[no_mangle]
pub extern fn rust_main() {

    let hello = b"Hello World!";

    let mut hello_colored = [0x1f; 24];
	for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    //write `Hello World!` to the center of the VGA text buffer
	let buffer = 0xb8000 as *mut _;
    unsafe { *buffer = hello_colored };

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
