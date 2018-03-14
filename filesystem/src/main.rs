// #![feature(lang_items)]
// #![feature(start)]
// #![no_std]

//#[start]
// fn main(argc: isize, args: *const *const u8) -> isize {
//     return 5;
// }

fn main() {
    println!("Test");
}

// #[lang = "eh_personality"] extern fn eh_personality() { }

// #[no_mangle]
// #[lang = "panic_fmt"]
// pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! { loop {} }