use memory::gdt::SegmentDescriptorTable;
use memory::segmentation::{ SegmentDescriptor };
use alloc::vec::Vec;
use super::elf::*;
use super::loader::LoadInformation;

#[repr(packed)]
#[derive(Debug, Default)]
pub struct TaskStateSegment {
    pub link: u32,
    pub esp0: u32,
    pub ss0: u32,
    pub esp1: u32,
    pub ss1: u32,
    pub esp2: u32,
    pub ss2: u32,
    pub cr3: u32,
    pub reserved: [u32; 16],
    pub ldtr: u32,
    pub res: u16,
    pub iopb_offset: u16,
}

#[repr(packed, C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct CpuState {
    pub eip: u32, //1
    pub eflags: u32, //2
    pub cs: u32, //3
    pub ds: u32, //4
    pub ebp: u32, //5
    pub esp: u32, //6
    pub edi: u32, //7
    pub esi: u32, //8
    pub edx: u32, //9
    pub ecx: u32, //10
    pub ebx: u32, //11
    pub eax: u32, //12
}

pub struct Process {
    pub executable_file: ElfFile,
    load_information: Option<LoadInformation>,
    cpu_state: CpuState,
    ldt: SegmentDescriptorTable,
    tss: TaskStateSegment,
}

impl Process {
    pub fn new(executable_file: ElfFile) -> Self {
        Process {
            executable_file: executable_file,
            load_information: None,
            cpu_state: CpuState::default(),
            ldt: SegmentDescriptorTable::new(),
            tss: TaskStateSegment::default(),
        }
    }

    pub fn set_load_information(&mut self, load_information: LoadInformation) {
        self.load_information = Some(load_information);
    }

    pub fn get_load_information(&self) -> &LoadInformation {
        self.load_information.as_ref().unwrap()
    }

    pub fn get_elf_header(&self) -> &ElfHeader {
        &self.executable_file.elf_header
    }

    pub fn set_ldt_descriptors(&mut self, descriptors: &Vec<SegmentDescriptor>) {
        self.ldt.set_descriptors(descriptors);
    }

    pub fn get_ldt(&self) -> &SegmentDescriptorTable {
        &self.ldt
    }

    pub fn set_kernel_stack(&mut self, ss0: u16, esp0: u32) {
        self.tss.ss0 = ss0 as u32;
        self.tss.esp0 = esp0;
        self.tss.iopb_offset = 104;
    }

    pub fn get_tss(&self) -> &TaskStateSegment {
        &self.tss
    }

    pub fn get_cpu_state(&self) -> &CpuState {
        &self.cpu_state
    }

    pub fn set_cpu_state(&mut self, cpu_state: CpuState) {
        self.cpu_state = cpu_state;
    }
}