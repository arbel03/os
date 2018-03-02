// ELF-32 bit implementation

pub struct ElfHeader {
    magic: [u8; 4],
    class: u8,
    endianness: u8,
    version: u8, 
    os_abi: u8,
    abi_version: u8,
    unused: [u8; 7],
    type: u16,
    machine: u16,
    version2: u32,
    entry_point: u32,
    phoff: u32, // Program Header offset
    shoff: u32 // Section Header offset
    flags: u32,
    header_size: u16,
    phentsize: u16, // Program Header entry size
    phnum: u16, // Program Header entry count
    shentsize: u16, // Section Header entry size
    shnum: u16, // Section Header entry count
    e_shstrndx: u16,
}

#[repr(packed, u32)]
pub enum ProgramEntryType {
    PT_NULL = 0x00000000,
    PT_LOAD = 0x00000001,
    PT_DYNAMIC = 0x00000002,
    PT_INTERP = 0x00000003,
    PT_NOTE = 0x00000004,
    PT_SHLIB = 0x00000005,
    PT_PHDR = 0x00000006,
    PT_LOOS = 0x60000000,
    PT_HIOS = 0x6FFFFFFF,
    PT_LOPROC = 0x70000000,
    PT_HIPROC = 0x7FFFFFFF,
}

pub struct ProgramHeaderEntry {
    type: ProgramEntryType,
    offset: u32, // offset to segment in file image
    vaddr: u32, // Virtual address in memory
    paddr: u32, // Physical address in memory
    file_size: u32, // Size of segment in file
    mem_size: u32, // Size of segment in memory
    flags: u32,
    align: u32,
}