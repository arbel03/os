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
    phoff: u32, // Program Header offset
    shoff: u32, // Section Header offset
    flags: u32,
    header_size: u16,
    phentsize: u16, // Program Header entry size
    phnum: u16, // Program Header entry count
    shentsize: u16, // Section Header entry size
    shnum: u16, // Section Header entry count
    e_shstrndx: u16,
}

#[repr(u32)]
#[allow(dead_code)]
pub enum ProgramEntryType {
    PtNull = 0x00000000,
    PtLoad = 0x00000001,
    PtDynamic = 0x00000002,
    PtInterp = 0x00000003,
    PtNote = 0x00000004,
    PtShlib = 0x00000005,
    PtPhdr = 0x00000006,
    PtLoos = 0x60000000,
    PtHios = 0x6FFFFFFF,
    PtLoproc = 0x70000000,
    PtHiproc = 0x7FFFFFFF,
}

pub struct ProgramHeaderEntry {
    entry_type: ProgramEntryType,
    offset: u32, // offset to segment in file image
    vaddr: u32, // Virtual address in memory
    paddr: u32, // Physical address in memory
    file_size: u32, // Size of segment in file
    mem_size: u32, // Size of segment in memory
    flags: u32,
    align: u32,
}