use super::PROCESS_ALLOCATOR;
use super::elf::*;
use core::slice;
use alloc::Vec;
use alloc::allocator::{ Layout, Alloc };
use memory::segmentation::SegmentDescriptor;
use super::process::Process;

#[derive(Debug)]
pub struct LoadInformation {
    process_base: *const u8,
    arguments_start: *const u8,
    stack_pointer: *const u8,
}

impl LoadInformation {
    pub fn translate_virtual_to_physical_address(&self, address: *const u8) -> *const u8 {
        (address as usize + self.process_base as usize) as *const u8
    }

    pub fn translate_physical_to_virtual_address(&self, address: *const u8) -> *const u8 {
        (address as usize - self.process_base as usize) as *const u8
    }
}

pub enum LoadError {
    MissingSegment,
    NoMemory,
}

pub(in super) unsafe fn load_process(process: &Process, args: &[&str], load_request: LoadRequest) -> Result<LoadInformation, LoadError> {
    use syscall::fs::{ seek, read };
    
    let process_size_total = load_request.get_total_process_size();
    
    // Allocating process
    let layout = Layout::from_size_align_unchecked(process_size_total, 1);
    let process_base: *mut u8;
    match (&PROCESS_ALLOCATOR).alloc(layout) {
        Ok(ptr) => process_base = ptr,
        Err(error) => {
            use alloc::allocator::AllocErr;
            match error {
                AllocErr::Exhausted { request } => return Err(LoadError::NoMemory),
                _ => panic!("Process allocator error."),
            };
        }
    };

    let process_limit = (process_base as usize + process_size_total as usize) as *const u8;
    let mut ldt_entries: Vec<SegmentDescriptor> = Vec::new();
    let file_descriptor = process.executable_file.get_file_descriptor();
    for segment in process.executable_file.get_program_header_entries() {
        if segment.entry_type.get_type() == EntryType::PtLoad {
            // Segment is an executable segment
            if segment.flags & Flags::Executable as u32 == Flags::Executable as u32 {
                // Adding a new user space code descriptor
                ldt_entries.insert(0, SegmentDescriptor::new(process_base as u32, process_limit as u32, 0b11111010, 0b0100));    
            } else {
                // Adding a new user space data descriptor
                ldt_entries.push(SegmentDescriptor::new(process_base as u32, process_limit as u32, 0b11110010, 0b0100));
            }

            let slice = slice::from_raw_parts_mut((segment.vaddr as usize + process_base as usize) as *mut u8, segment.file_size as usize);
            seek(file_descriptor, segment.offset as usize);
            read(file_descriptor, slice);
        }
    }

    Ok(LoadInformation {
        process_base: process_base as *const u8,
        arguments_start: 0 as *const u8,
        stack_pointer: 0 as *const u8,
    })
}

#[derive(Debug)]
pub struct LoadRequest {
    process_area_size: usize,
    stack_area_size: usize,
    arguments_count: usize,
    arguments_area_size: usize,
}

impl LoadRequest {
    pub fn get_total_process_size(&self) -> usize {
        use core::mem;
        self.process_area_size + self.stack_area_size + self.arguments_area_size + self.arguments_count * mem::size_of::<*const u8>()
    }

    pub fn get_arguments_start(&self) -> *mut u8 {
        return (self.process_area_size + self.stack_area_size) as *mut u8;
    }
}

pub(in super) unsafe fn create_load_request(process: &Process, args: &[&str]) -> LoadRequest {
    let mut process_size = 0;
    for entry in process.executable_file.get_program_header_entries() {
        if entry.entry_type.get_type() == EntryType::PtLoad {
            if entry.vaddr + entry.mem_size >= process_size {
                process_size = entry.vaddr + entry.mem_size;
            }
        }
    }

    let mut args_length = 0;
    for arg in args {
        args_length += arg.len() + 1;
    }

    LoadRequest {
        process_area_size: process_size as usize,
        stack_area_size: 1024*50,
        arguments_count: args.len(),
        arguments_area_size: args_length,
    }
}

#[derive(Debug)]
pub enum CreationError {
    ExecutableNotFound,
    InvalidElfHeader,
}

pub(in super) unsafe fn create_process(executable_path: &str) -> Result<Process, CreationError> {
    use syscall::fs::open;

    let fd: usize = open(executable_path);
    let header = ElfFile::read_elf_header(fd);
    if !header.is_valid() {
        return Err(CreationError::InvalidElfHeader);
    }
    let entries = ElfFile::read_program_header_entries(fd, &header);
    let elf_file = ElfFile::new(executable_path, fd, header, entries);
    let process = Process::new(elf_file);
    return Ok(process);
}