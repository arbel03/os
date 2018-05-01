use super::PROCESS_ALLOCATOR;
use super::elf::*;
use core::slice;
use alloc::Vec;
use alloc::allocator::{ Layout, Alloc };
use memory::segmentation::SegmentDescriptor;
use super::process::Process;

#[derive(Debug)]
pub struct LoadInformation {
    pub process_base: *const u8,
    pub stack_pointer: *mut u8,
    pub argument_pointers_start: *const *const u8,
    pub arguments_count: usize,
    pub ldt_entries: Vec<SegmentDescriptor>,
}

impl LoadInformation {
    pub fn translate_virtual_to_physical_address(&self, address: *const u8) -> *const u8 {
        (address as usize + self.process_base as usize) as *const u8
    }

    pub fn translate_physical_to_virtual_address(&self, address: *const u8) -> *const u8 {
        (address as usize - self.process_base as usize) as *const u8
    }

    pub fn get_ldt_entries(&self) -> &Vec<SegmentDescriptor> {
        &self.ldt_entries
    }
}

#[derive(Debug)]
pub enum LoadError {
    MissingSegment,
    NoMemory,
}

pub(in super) unsafe fn load_process(process: &Process, args: &[&str], load_request: LoadRequest) -> Result<LoadInformation, LoadError> {    
    use syscall::{ read, seek };
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

    let process_limit = (process_base as usize + process_size_total as usize) as u32;
    let mut ldt_entries: Vec<SegmentDescriptor> = Vec::new();
    let file_descriptor = process.executable_file.get_file_descriptor();
    for segment in process.executable_file.get_program_header_entries() {
        if segment.entry_type.get_type() == EntryType::PtLoad {
            // Segment is an executable segment
            if segment.flags & Flags::Executable as u32 == Flags::Executable as u32 {
                // Adding a new user space code descriptor
                ldt_entries.insert(0, SegmentDescriptor::new(process_base as u32, process_limit/0x1000+1, 0b11111010, 0b1100));    
            } else {
                // Adding a new user space data descriptor
                ldt_entries.push(SegmentDescriptor::new(process_base as u32, process_limit/0x1000+1, 0b11110010, 0b1100));
            }

            let slice = slice::from_raw_parts_mut((segment.vaddr as usize + process_base as usize) as *mut u8, segment.file_size as usize);
            seek(file_descriptor, segment.offset as usize);
            read(file_descriptor, slice);
        }
    }

    // Copying arguments to the process's address space.
    let arguments_start = load_request.get_arguments_start();
    let pointers_start = load_request.get_pointer_array_start();

    let arguments_start_physical = process_base.offset(arguments_start as isize);
    let pointers_start_physical = process_base.offset(pointers_start as isize) as *mut *const u8;

    let pointers_slice = slice::from_raw_parts_mut(pointers_start_physical, load_request.get_argument_count());
    let mut arg_offset: usize = 0;
    for (index, arg) in args.iter().enumerate() {
        let arg_slice = slice::from_raw_parts_mut(arguments_start_physical.offset(arg_offset as isize), arg.len());
        arg_slice.clone_from_slice(arg.as_bytes());
        pointers_slice[index] = arguments_start.offset(arg_offset as isize);
        *(arg_slice.as_mut_ptr().offset(arg_slice.len() as isize)) = 0;
        arg_offset += arg.len() + 1;
    }

    let stack_pointer = load_request.get_stack_pointer();

    Ok(LoadInformation {
        process_base: process_base as *const u8,
        stack_pointer: stack_pointer,
        argument_pointers_start: pointers_start as *const *const u8,
        arguments_count: args.len(),
        ldt_entries: ldt_entries,
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

    pub fn get_pointer_array_start(&self) -> *mut *const u8 {
        return (self.process_area_size + self.stack_area_size + self.arguments_area_size) as *mut *const u8;
    }

    pub fn get_stack_pointer(&self) -> *mut u8 {
        (self.process_area_size + self.stack_area_size) as *mut u8
    }

    pub fn get_argument_count(&self) -> usize {
        self.arguments_count
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
        stack_area_size: 1024*10,
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
    use syscall::open;

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