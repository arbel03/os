use memory::gdt::SegmentDescriptorTable;
use memory::segmentation::{ SegmentDescriptor, SegmentSelector };
use alloc::vec::Vec;

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

enum ProcessState {
    New,
    Ready,
    Running,
    Blocked,
}

// TODO: Add TSS
pub struct Process {
    process_state: ProcessState,
    address_space: Vec<SegmentDescriptor>,
    ldt: SegmentDescriptorTable,
    tss: TaskStateSegment,
}

impl Process {
    pub fn new() -> Self {
        Process {
            process_state: ProcessState::New,
            ldt: SegmentDescriptorTable::new(),
            address_space: Vec::new(),
            tss: TaskStateSegment::empty(),
        }
    }

    pub fn get_ldt(&self) -> &SegmentDescriptorTable {
        &self.ldt
    }

    pub fn get_tss(&mut self) -> &mut TaskStateSegment {
        &mut self.tss
    }

    pub fn set_ldt_descriptors(&mut self, descriptors: Vec<SegmentDescriptor>) {
        // self.ldt.init_with_length(3);
        self.address_space = descriptors;
        self.ldt.set_descriptors(&self.address_space);
    }

    pub fn translate_data_address(&self, virtual_address: u32) -> u32 {
        return self.address_space[1].base + virtual_address;
    }

    pub fn setup_process(&mut self, ss0: u32, esp0: u32, entry_point: u32, esp: u32) {
        self.tss.ss0 = ss0;
        self.tss.esp0 = esp0;
        self.tss.eip = entry_point;
        self.tss.esp = esp;

        use memory::segmentation::TableType;
        self.tss.ss = SegmentSelector::new(2, TableType::LDT, 3) as u32;
        // Data segments
        self.tss.ds = SegmentSelector::new(1, TableType::LDT, 3) as u32;
        self.tss.gs = SegmentSelector::new(1, TableType::LDT, 3) as u32;
        self.tss.fs = SegmentSelector::new(1, TableType::LDT, 3) as u32;
        self.tss.es = SegmentSelector::new(1, TableType::LDT, 3) as u32;
        
        self.tss.cs = SegmentSelector::new(0, TableType::LDT, 3) as u32;
    }
}