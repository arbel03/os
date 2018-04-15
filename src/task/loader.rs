use super::PROCESS_ALLOCATOR;
use super::elf::*;
use syscall::fs::{ open, read, seek };
use core::{ slice, str };
use alloc::Vec;
use alloc::allocator::{ Layout, Alloc };
use memory::segmentation::SegmentDescriptor;

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

unsafe fn alloc_segment(size: usize, align: usize) -> *mut u8 {
    // Alloc space for the new process.
    let layout = Layout::from_size_align(size, align).unwrap();
    let ptr = (&*PROCESS_ALLOCATOR.as_mut().unwrap()).alloc(layout).unwrap();
    ptr
}

unsafe fn load_segments(fd: usize, entries: Vec<ProgramHeaderEntry>) -> Vec<SegmentDescriptor> {
    let mut segments: Vec<SegmentDescriptor> = Vec::new();

    for entry in entries.iter() {
        if entry.entry_type.get_type() == EntryType::PtLoad {
            // Segment is an executable segment
            let ptr = if entry.flags & 0x01 == 0x01 {
                // Allocating the needed size
                let ptr = alloc_segment((entry.mem_size + entry.vaddr) as usize, entry.align as usize);
                
                // Adding a new user space code descriptor
                segments.insert(0, SegmentDescriptor::new(ptr as u32, ptr as u32 + entry.vaddr + entry.mem_size, 0b11111010, 0b0100));    

                ptr
            } else {
                // Allocating the needed size + stack size
                let ptr = alloc_segment((entry.mem_size + entry.vaddr + 1024*50) as usize, entry.align as usize);

                // Adding a new user space data descriptor
                segments.push(SegmentDescriptor::new(ptr as u32, ptr as u32 + (entry.mem_size + entry.vaddr + 1024*50 as u32), 0b11110010, 0b0100));
                
                ptr
            };

            let slice = slice::from_raw_parts_mut((entry.vaddr as usize + ptr as usize) as *mut u8, entry.file_size as usize);
            seek(fd, entry.offset as usize);
            read(fd, slice);
        }
    }

    // First is null, second is code, third is data.
    return segments;
}

pub(in super) unsafe fn load_elf(file_name: &str) -> (ElfHeader, Vec<SegmentDescriptor>) {
    let mut elf_header = ElfHeader::default();
    let entries: Vec<ProgramHeaderEntry>;
    let fd: usize;
    fd = open(file_name);
    read_header(fd, &mut elf_header);
    if !elf_header.is_valid() {
        panic!("Not a valid ELF file.");
    }

    entries = read_ph_entries(fd, &elf_header);
    // First is null, second is code, third is data
    let segments = load_segments(fd, entries);

    return (elf_header, segments);
}

