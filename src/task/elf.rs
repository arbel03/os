use alloc::Vec;
use alloc::String;

// ELF-32 bit implementation

#[derive(Default, Debug)]
#[repr(packed)]
pub struct ElfHeader {
    magic: [u8; 4],
    class: u8,
    endianness: u8,
    version: u8, 
    os_abi: u8,
    abi_version: u8,
    unused: [u8; 7],
    elf_type: u16,
    machine: u16,
    version2: u32,
    pub entry_point: u32,
    pub phoff: u32, // Program Header offset
    shoff: u32, // Section Header offset
    flags: u32,
    header_size: u16,
    pub phentsize: u16, // Program Header entry size
    pub phnum: u16, // Program Header entry count
    shentsize: u16, // Section Header entry size
    shnum: u16, // Section Header entry count
    e_shstrndx: u16,
}

impl ElfHeader {
    pub fn is_valid(&self) -> bool {
        self.magic == [0x7f, 0x45, 0x4c, 0x46]
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum EntryType {
    PtNull,
    PtLoad,
    PtDynamic,
    PtInterp,
    PtNote,
    PtShlib,
    PtPhdr,
    Unknown,
    PtLoHiproc(u32),
    PtLoHios(u32),
}

#[repr(u32)]
#[allow(dead_code)]
pub enum Flags {
    Executable = 0x1,
    Writeable = 0x2,
    Readable = 0x4,
}

#[derive(Clone, Copy, Debug)]
#[repr(packed, C)]
pub struct ProgramEntryType(u32);

impl ProgramEntryType {
    pub fn get_type(&self) -> EntryType {
        let val = self.0;
        match val {
            0x00000000 => EntryType::PtNull,
            0x00000001 => EntryType::PtLoad,
            0x00000002 => EntryType::PtDynamic,
            0x00000003 => EntryType::PtInterp,
            0x00000004 => EntryType::PtNote,
            0x00000005 => EntryType::PtShlib,
            0x00000006 => EntryType::PtPhdr,
            0x70000000...0x7FFFFFFF => EntryType::PtLoHiproc(val),
            0x60000000...0x6FFFFFFF => EntryType::PtLoHios(val),
            _ => EntryType::Unknown,
        }
    }
}

#[repr(packed, C)]
#[derive(Debug, Clone)]
pub struct ProgramHeaderEntry {
    pub entry_type: ProgramEntryType,
    pub offset: u32, // offset to segment in file image
    pub vaddr: u32, // Virtual address in memory
    pub paddr: u32, // Physical address in memory
    pub file_size: u32, // Size of segment in file
    pub mem_size: u32, // Size of segment in memory
    pub flags: u32,
    pub align: u32,
}

impl ProgramHeaderEntry {
    pub const fn empty() -> Self {
        ProgramHeaderEntry {
            entry_type: ProgramEntryType(0),
            offset: 0, 
            vaddr: 0, 
            paddr: 0, 
            file_size: 0, 
            mem_size: 0,
            flags: 0,
            align: 0,
        }
    }
}

pub struct ElfFile {
    file_name: String,
    file_descriptor: usize,
    elf_header: ElfHeader,
    program_header_entries: Vec<ProgramHeaderEntry>,
}

impl ElfFile {
    pub fn new(file_name: &str, file_descriptor: usize, header: ElfHeader, entries: Vec<ProgramHeaderEntry>) -> Self {
        use alloc::string::ToString;
        ElfFile {
            file_name: file_name.to_string(),
            file_descriptor: file_descriptor,
            elf_header: header,
            program_header_entries: entries,
        }
    }

    pub fn get_file_descriptor(&self) -> usize {
        self.file_descriptor
    }

    pub fn get_program_header_entries(&self) -> &Vec<ProgramHeaderEntry> {
        &self.program_header_entries
    }

    pub unsafe fn read_elf_header(fd: usize) -> ElfHeader {
        use syscall::fs::{ read, seek };
        use core::slice::from_raw_parts_mut;

        let mut header = ElfHeader::default();
        let read_buff = from_raw_parts_mut(&mut header as *mut ElfHeader as *mut u8, 52);
        seek(fd, 0);
        read(fd, read_buff);
        return header;
    }

    pub unsafe fn read_program_header_entries(file_descriptor: usize, header: &ElfHeader) -> Vec<ProgramHeaderEntry> {
        use syscall::fs::{ read, seek };
        use core::slice::from_raw_parts_mut;

        let ph_entries = vec![ProgramHeaderEntry::empty(); header.phnum as usize];
        seek(file_descriptor, header.phoff as usize);
        
        let buff_slice = from_raw_parts_mut(ph_entries.as_ptr() as *mut u8, (header.phentsize*header.phnum) as usize);
        read(file_descriptor, buff_slice);

        return ph_entries;
    }
}