use syscall::fs::{ open, read, seek };
use super::elf::*;
use core::slice;
use core::str;
use alloc::Vec;

unsafe fn read_header(file_descriptor: usize, header: &mut ElfHeader) {
    let read_buff = slice::from_raw_parts_mut(header as *mut ElfHeader as *mut u8, 52);
    seek(file_descriptor, 0);
    read(file_descriptor, read_buff);
}

unsafe fn read_ph_entries(file_descriptor: usize, header: &ElfHeader) -> Vec<ProgramHeaderEntry> {
    let ph_entries = vec![ProgramHeaderEntry::empty(); header.phnum as usize];
    seek(file_descriptor, header.phoff as usize);
    
    let buff_slice = slice::from_raw_parts_mut(ph_entries.as_ptr() as * mut u8, (header.phentsize*header.phnum) as usize);
    read(file_descriptor, buff_slice);

    return ph_entries;
}

pub fn load_elf(file_name: &str) {
    // Test filesystem syscalls
    let mut elf_header = ElfHeader::default();
    let entries: Vec<ProgramHeaderEntry>;
    unsafe {
        let fd = open(file_name);
        read_header(fd, &mut elf_header);
        entries = read_ph_entries(fd, &elf_header);
    }
    //println!("Elf header of {}\n{:?}", file_name, elf_header);
    println!("Entries count: {}", &entries.len());
    for (index, entry) in entries.iter().enumerate() {
        println!("Entry #{} {:?}", index, entry);
    }
}

