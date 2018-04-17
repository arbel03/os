#![feature(alloc)]
#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;
extern crate std;

#[no_mangle]
pub fn main(argc: usize, args: &str) {
    let fd = std::fs::open(args);
    if fd != 0xffffffff {
        std::io::printf("Printing contents of file \"");
        std::io::printf(args);
        std::io::printf("\":\n");
        let file_size = std::fs::filesz(fd);
        let mut vector = vec![0u8;file_size];
        std::fs::read(fd, &mut vector);
        use core::str;
        std::io::printf(unsafe { str::from_utf8_unchecked(&vector) });
    } else {
        std::io::printf("Error opening file.\n");
    }
}