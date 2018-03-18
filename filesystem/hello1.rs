// #![feature(lang_items)]
// #![feature(start)]
// #![no_std]

// #[start]
fn main() {
    println!("Hello");
}

// #[lang = "eh_personality"] extern fn eh_personality() { }

// #[no_mangle]
// #[lang = "panic_fmt"]
// pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! { loop {} }