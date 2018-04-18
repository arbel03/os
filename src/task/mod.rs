pub mod process;
mod loader;
mod elf;

use self::process::*;
use BitmapAllocator;
use alloc::boxed::Box;
use alloc::Vec;
use memory::MemoryArea;

static mut PROCESS_ALLOCATOR: BitmapAllocator = BitmapAllocator::new(0x0, 0x0, 0x0);
pub static mut CURRENT_PROCESS: Option<Box<Process>> = None;

pub fn init(free_memory_areas: Vec<MemoryArea>) {
    // Set up an allocator for the process area
    if free_memory_areas.len() > 0 {
        let process_area = free_memory_areas[0];
        println!("Allocating processes from {:#x} to {:#x}.", process_area.base, process_area.base+process_area.size);
        unsafe {
            PROCESS_ALLOCATOR.set_bitmap_start(process_area.base);
            PROCESS_ALLOCATOR.set_block_size(process_area.size/100);
            PROCESS_ALLOCATOR.set_size(process_area.size);
            PROCESS_ALLOCATOR.init();
        }
    } else {
        panic!("No space for process allocator.");
    }
}

pub unsafe fn execv(file_name: &str, args: &[&str]) {
    use memory::segmentation::{ SegmentSelector, TableType };
    use memory::gdt::{ Gdt, DescriptorType };
    use memory::GDT;
    
    let process = loader::create_process(file_name).expect("An error occured.");
    let mut boxed_process = Box::new(process);
    let process = boxed_process.as_mut();

    let load_request = loader::create_load_request(&process, args);
    println!("{:?}", load_request);
    match loader::load_process(&process, args, load_request) {
        Ok(load_information) => println!("{:?}", load_information),
        Err(LoadError) => println!("Load error."),
    }

    // let (code_selector, data_selector, stack_pointer) = {
    //     let process_base = loaded_segments[0].base as usize;
    //     println!("Loading process at {}", process_base);


    //     let code_selector = SegmentSelector::new(0, TableType::LDT, 3);
    //     let data_selector = SegmentSelector::new(1, TableType::LDT, 3);
    //     let mut stack_pointer = loaded_segments[1].limit;

    //     let args_length = {
    //         let mut length = 0;
    //         for arg in args {
    //             length += arg.len() + 1;
    //         }
    //         length
    //     };

    //     use core::slice;

    //     let pointers_length = args.len() * ::core::mem::size_of::<usize>();
    //     let args_start = stack_pointer as usize - args_length;
    //     let pointers_start = stack_pointer as usize - args_length - pointers_length;
    //     let mut pointers = slice::from_raw_parts_mut(pointers_start as *mut *const u8, args.len());
        
    //     let mut arg_offset = 0;
    //     let mut pointer_index = 0;
    //     for arg in args {
    //         let current_arg_offset = args_start + arg_offset + process_base;
    //         let arg_length = arg.len();
    //         let new_arg_slice = slice::from_raw_parts_mut(current_arg_offset as *mut u8, arg_length);
    //         let arg_slice = slice::from_raw_parts(arg.as_ptr(), arg_length);
    //         new_arg_slice.clone_from_slice(arg_slice);
    //         // new_arg_slice[arg_length] = 0x00;
    //         pointers[pointer_index] = (current_arg_offset - process_base) as *const u8;
    //         pointer_index += 1;
    //         arg_offset += arg.len() + 1;
    //     }

    //     // Updating stack pointer
    //     stack_pointer -= (pointers_length + args_length + 8) as u32;

    //     asm!("
    //     mov eax, esp
    //     mov esp, $0
    //     push $1
    //     push $2
    //     mov esp, eax
    //     " :: 
    //     "{edx}"(stack_pointer + process_base as u32),
    //     "{ebx}"(args_start - process_base),
    //     "{ecx}"(args.len())
    //     :: "intel");

    //     stack_pointer -= 12;

    //     // Set process information
    //     process.setup_process(GDT.get_selector(DescriptorType::KernelData, 0), 0x9fc00, elf_header.entry_point, stack_pointer, code_selector, data_selector);

    //     // Set LDT in process information block
    //     process.set_ldt_descriptors(loaded_segments);

    //     // Set LDT in GDT
    //     GDT.set_ldt(process.get_ldt());
    //     // process.get_tss().ldtr = GDT.get_selector(DescriptorType::LdtDescriptor, 0) as u32;
    //     // Set TSS
    //     GDT.set_tss(process.get_tss());

    //     asm!("ltr $0" :: "r"(GDT.get_selector(DescriptorType::TssDescriptor, 0) as u16) :: "intel");
    //     asm!("lldt $0" :: "r"(GDT.get_selector(DescriptorType::LdtDescriptor, 0) as u16) :: "intel");

    //     (code_selector, data_selector, stack_pointer)
    // };

    // // println!("\nCODE: {:#x}\nDATA: {:#x}\nESP: {:#x}\nENTRY: {:#x}\n", code_selector, data_selector, stack_pointer, elf_header.entry_point);

    // CURRENT_PROCESS = Some(boxed_process);

    // // Perform context switch to loaded task.
    // asm!("
    // push $0
    // push $1
    // pushfd
    // push $2
    // push $3
    // mov eax, $0
    // mov ds, ax
    // mov fs, ax
    // mov gs, ax
    // mov es, ax
    // iretd
    // " ::
    // "r"(data_selector as u32),
    // "r"(stack_pointer),
    // "r"(code_selector as u32)
    // "r"(elf_header.entry_point)
    // :: "intel", "volatile");
}
