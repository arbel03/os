use syscall::fs::{ open, read, seek };
use super::elf::*;
use core::slice;
use core::str;

unsafe fn read_header(file_descriptor: usize, header: &mut ElfHeader) {
    let read_buff = slice::from_raw_parts_mut(header as *mut ElfHeader as *mut u8, 52);
    seek(file_descriptor, 0);
    read(file_descriptor, read_buff);
}

pub fn load_elf(file_name: &str) {
    // Test filesystem syscalls
    let mut elf_header = ElfHeader::default();
    unsafe {
        let fd = open(file_name);
        read_header(fd, &mut elf_header);
    }
    println!("Elf header of {}\n{:?}", file_name, elf_header);
}

