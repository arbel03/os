use syscall::fs::{ open, read, seek };
use super::elf::*;
use core::slice;
use core::str;
use alloc::Vec;
use super::PROCESS_ALLOCATOR;
use alloc::allocator::{ Layout, Alloc };

unsafe fn read_header(file_descriptor: usize, header: &mut ElfHeader) {
    let read_buff = slice::from_raw_parts_mut(header as *mut ElfHeader as *mut u8, 52);
    seek(file_descriptor, 0);
    read(file_descriptor, read_buff);
}

unsafe fn read_ph_entries(file_descriptor: usize, header: &ElfHeader) -> Vec<ProgramHeaderEntry> {
    let ph_entries = vec![ProgramHeaderEntry::empty(); header.phnum as usize];
    seek(file_descriptor, header.phoff as usize);
    
    let buff_slice = slice::from_raw_parts_mut(ph_entries.as_ptr() as *mut u8, (header.phentsize*header.phnum) as usize);
    read(file_descriptor, buff_slice);

    return ph_entries;
}

pub unsafe fn load_segments(fd: usize, entries: Vec<ProgramHeaderEntry>) {
    for entry in entries[..2].iter() {
        if entry.entry_type.get_type() == EntryType::PtLoad {
            // Alloc space for the new process.
            let layout = Layout::from_size_align(entry.mem_size as usize, entry.align as usize).unwrap();
            let ptr = PROCESS_ALLOCATOR.as_mut().unwrap().alloc(layout).unwrap();

            // Loading segment from disk to memory.
            let slice = slice::from_raw_parts_mut((entry.vaddr as usize + ptr as usize) as *mut u8, entry.file_size as usize);
            print!("Loading segment starting at {:?}, size: {:#x}.", slice.as_ptr(), slice.len());
            seek(fd, entry.offset as usize);
            read(fd, slice);
            println!(" Loaded.");
        }
    }
}

pub unsafe fn load_elf(file_name: &str) {
    // Test filesystem syscalls
    let mut elf_header = ElfHeader::default();
    let entries: Vec<ProgramHeaderEntry>;
    let fd: usize;
    fd = open(file_name);
    read_header(fd, &mut elf_header);
    if !elf_header.is_valid() {
        panic!("Not a valid ELF file.");
    }
    entries = read_ph_entries(fd, &elf_header);
    load_segments(fd, entries);
}

