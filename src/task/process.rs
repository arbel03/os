use memory::gdt::SegmentDescriptorTable;
use memory::segmentation::{ SegmentDescriptor };
use alloc::vec::Vec;
use alloc::string::String;
use super::elf::*;
use super::loader::LoadInformation;

#[repr(packed)]
#[derive(Debug)]
pub struct TaskStateSegment {
    pub link: u32, // Next Tss entry
    pub esp0: u32, // Stacks
    pub ss0: u32,
    pub esp1: u32,
    pub ss1: u32,
    pub esp2: u32,
    pub ss2: u32,
    pub cr3: u32,
    pub eip: u32,
    pub eflags: u32,
    pub eax: u32,
    pub ecx: u32,
    pub edx: u32,
    pub ebx: u32,
    pub esp: u32, 
    pub ebp: u32,
    pub esi: u32,
    pub edi: u32,
    pub es: u32, // Segments
    pub cs: u32,
    pub ss: u32,
    pub ds: u32,
    pub fs: u32,
    pub gs: u32,
    pub ldtr: u32,
    pub res: u16,
    pub iopb_offset: u16,
}

impl TaskStateSegment {
    pub const fn empty() -> Self {
        TaskStateSegment {
            link: 0, 
            esp0: 0, 
            ss0: 0,
            esp1: 0,
            ss1: 0,
            esp2: 0,
            ss2: 0,
            cr3: 0,
            eip: 0,
            eflags: 0,
            eax: 0,
            ecx: 0,
            edx: 0,
            ebx: 0,
            esp: 0, 
            ebp: 0,
            esi: 0,
            edi: 0,
            es: 0,
            cs: 0,
            ss: 0,
            ds: 0,
            fs: 0,
            gs: 0,
            ldtr: 0,
            res: 0,
            iopb_offset: 0,
        }
    }
}

pub struct Process {
    pub executable_file: ElfFile,
    load_information: Option<LoadInformation>,
    ldt: SegmentDescriptorTable,
    tss: TaskStateSegment,
}

impl Process {
    pub fn new(executable_file: ElfFile) -> Self {
        Process {
            executable_file: executable_file,
            load_information: None,
            ldt: SegmentDescriptorTable::new(),
            tss: TaskStateSegment::empty(),
        }
    }

    pub fn set_load_information(&mut self, load_information: LoadInformation) {
        self.load_information = Some(load_information);
    }

    pub fn get_load_information(&self) -> &LoadInformation {
        self.load_information.as_ref().unwrap()
    }

    pub fn get_elf_header(&mut self) -> &ElfHeader {
        &self.executable_file.elf_header
    }

    pub fn set_ldt_descriptors(&mut self, descriptors: &Vec<SegmentDescriptor>) {
        self.ldt.set_descriptors(descriptors);
    }

    pub fn get_ldt(&self) -> &SegmentDescriptorTable {
        &self.ldt
    }

    pub fn setup_process(&mut self, ss0: u16, esp0: u32) {
        self.tss.ss0 = ss0 as u32;
        self.tss.esp0 = esp0;
        self.tss.iopb_offset = 104;
    }

    pub fn get_tss(&mut self) -> &TaskStateSegment {
        &mut self.tss
    }
}