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
    let fd: usize;
    unsafe {
        fd = open(file_name);
        read_header(fd, &mut elf_header);
        if !elf_header.is_valid() {
            panic!("Not a valid ELF file.");
        }
        entries = read_ph_entries(fd, &elf_header);
    }

    println!("Entry Address: {:#x}", elf_header.entry_point);
    for entry in entries.iter() {
        let entry_type = entry.entry_type.get_type();
        if entry_type == EntryType::PtLoad {
            // Loading segment from disk to memory.
            let slice = unsafe { slice::from_raw_parts_mut(entry.vaddr as *mut u8, entry.file_size as usize) };
            println!("Section starting at {:?}, size: {:#x}", slice.as_ptr(), slice.len());
            // unsafe { 
            //     seek(fd, entry.offset as usize);
            //     read(fd, slice);
            // }
        }
    }

}

