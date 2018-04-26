use memory::gdt::SegmentDescriptorTable;
use memory::segmentation::{ SegmentDescriptor };
use alloc::vec::Vec;
use alloc::boxed::Box;
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

#[derive(Debug, Default)]
pub struct CpuState {
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
    pub es: u32,
    pub cs: u32,
    pub ss: u32,
    pub ds: u32,
    pub fs: u32,
    pub gs: u32
}

pub struct Process {
    pub executable_file: ElfFile,
    load_information: Option<LoadInformation>,
    parent_process: Option<Box<Process>>,
    cpu_state: CpuState,
    ldt: SegmentDescriptorTable,
    tss: TaskStateSegment,
}

impl Process {
    pub fn new(executable_file: ElfFile) -> Self {
        Process {
            executable_file: executable_file,
            load_information: None,
            parent_process: None,
            cpu_state: CpuState::default(),
            ldt: SegmentDescriptorTable::new(),
            tss: TaskStateSegment::default(),
        }
    }

    pub fn set_parent_process(&mut self, parent_process: Box<Process>) {
        self.parent_process = Some(parent_process);
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
        // self.tss.cs = cs as u32;
        // self.tss.ds = ds as u32;
        // self.tss.es = ds as u32;
        // self.tss.gs = ds as u32;
        // self.tss.fs = ds as u32;
        // self.tss.ss = ds as u32;
        // self.tss.eax = eax;
        // self.tss.ebx = ebx;
        // self.tss.ecx = ecx;
        // self.tss.edx = edx;
        // self.tss.edi = edi;
        // self.tss.esi = esi;
        // self.tss.ebp = ebp;
        // self.tss.eflags = eflags;
        // self.tss.eip = eip;
        // self.tss.esp = esp;
        self.cpu_state = cpu_state;
    }
}