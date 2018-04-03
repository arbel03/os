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
struct ProcessControlBlock {
    process_state: ProcessState,
}