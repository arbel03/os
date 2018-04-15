pub mod process;
mod loader;
mod elf;

use BitmapAllocator;
use self::process::*;
use alloc::boxed::Box;

static mut PROCESS_ALLOCATOR: Option<BitmapAllocator> = None;
pub static mut CURRENT_PROCESS: Option<Box<Process>> = None;

pub fn init(free_memory_areas: ::memory::MemoryAreas) {
    // Set up an allocator for the process area
    let process_area = free_memory_areas.0[0];
    println!("Allocating processes from {:#x} to {:#x}.", process_area.base, process_area.base+process_area.size);
    unsafe {
        PROCESS_ALLOCATOR = Some(BitmapAllocator::new(process_area.base, process_area.size, process_area.size/1000));
        PROCESS_ALLOCATOR.as_mut().unwrap().init();
    }
}

pub unsafe fn execv(file_name: &str) {
    use memory::segmentation::{ SegmentSelector, TableType };
    use memory::gdt::{ Gdt, DescriptorType };
    use memory::GDT;

    let (elf_header, loaded_segments) = loader::load_elf(file_name);

    // If only 3 segments, one is stack and the other is a null segment.
    // Every task should have one code segment.
    if loaded_segments.len() < 2 {
        panic!("Must have atleast one code segment and one data segment.");
    }

    let mut boxed_process = Box::new(Process::new());

    println!("{:?}", loaded_segments);

    let (code_selector, data_selector, stack_pointer) = {
        let process = boxed_process.as_mut();

        let code_selector = SegmentSelector::new(0, TableType::LDT, 3);
        let data_selector = SegmentSelector::new(1, TableType::LDT, 3);
        let stack_pointer = loaded_segments[1].limit;

        // Set process information
        process.setup_process(GDT.get_selector(DescriptorType::KernelData, 0), 0x9fc00, elf_header.entry_point, stack_pointer, code_selector, data_selector);

        // Set LDT in process information block
        process.set_ldt_descriptors(loaded_segments);

        // Set LDT in GDT
        GDT.set_ldt(process.get_ldt());
        // process.get_tss().ldtr = GDT.get_selector(DescriptorType::LdtDescriptor, 0) as u32;
        // Set TSS
        GDT.set_tss(process.get_tss());

        asm!("ltr $0" :: "r"(GDT.get_selector(DescriptorType::TssDescriptor, 0) as u16) :: "intel");
        asm!("lldt $0" :: "r"(GDT.get_selector(DescriptorType::LdtDescriptor, 0) as u16) :: "intel");

        (code_selector, data_selector, stack_pointer)
    };

    println!("\nCODE: {:#x}\nDATA: {:#x}\nESP: {:#x}\nENTRY: {:#x}\n", code_selector, data_selector, stack_pointer, elf_header.entry_point);

    CURRENT_PROCESS = Some(boxed_process);

    // Perform context switch to loaded task.
    asm!("
    push $0
    push $1
    pushfd
    push $2
    push $3
    mov eax, $0
    mov ds, ax
    mov fs, ax
    mov gs, ax
    mov es, ax
    mov ebp, $1
    iretd
    " ::
    "r"(data_selector as u32),
    "r"(stack_pointer),
    "r"(code_selector as u32)
    "r"(elf_header.entry_point)
    :: "intel", "volatile");
}
