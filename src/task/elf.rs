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
    entry_point: u32,
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

#[allow(dead_code)]
pub enum EntryType {
    PtNull,
    PtLoad,
    PtDynamic,
    PtInterp,
    PtNote,
    PtShlib,
    PtPhdr,
    PtLoproc,
    PtHiproc,
    Unknown,
    PtLoos(u32),
}

#[repr(packed)]
#[derive(Debug, Clone, Copy)]
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
            0x70000000 => EntryType::PtLoproc,
            0x7FFFFFFF => EntryType::PtHiproc,
            0x60000000...0x6FFFFFFF => EntryType::PtLoos(val),
            _ => EntryType::Unknown,
        }
    }
}

#[repr(packed)]
#[derive(Debug, Clone)]
pub struct ProgramHeaderEntry {
    pub entry_type: ProgramEntryType,
    offset: u32, // offset to segment in file image
    vaddr: u32, // Virtual address in memory
    paddr: u32, // Physical address in memory
    file_size: u32, // Size of segment in file
    mem_size: u32, // Size of segment in memory
    flags: u32,
    align: u32,
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