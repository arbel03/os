mod segmentation;
mod managment;
use BootloaderInfo;
use dtables::{TableDescriptor, lgdt};
use self::segmentation::*;

extern {
    fn gdt_flush();
}

// My gdt, containing 8 bytes entries (or unsigned 64 bit values)
static mut GDT: [u64; 5] = [0; 5];

pub unsafe fn encode_entry_at(index: usize, entry: SegmentDescriptor) {
    let mut descriptor_high: u32 = 0;
    // Create the high 32 bit segment
    descriptor_high  =  (entry.base & 0x00FF0000) >> 16;
    descriptor_high |= (entry.access_byte as u32) << 8;
    descriptor_high |= entry.limit & 0x000F0000;
    descriptor_high |=  (entry.flags as u32) << 20;
    descriptor_high |= entry.base & 0xFF000000;        

    let mut descriptor_low: u32 = 0;
    // Create the low 32 bit segment
    descriptor_low |= entry.base << 16;                 // set base bits 15:0
    descriptor_low |= entry.limit & 0xFFFF;               // set limit bits 15:0

    GDT[index] = (descriptor_high as u64) << 32;
    GDT[index] |= descriptor_low as u64;
}

pub fn init(bootloader_info: &BootloaderInfo) {
    unsafe {
        let code_segment = SegmentDescriptor::new(0, 0xffffffff, 0x9A, 0xC);
        let data_segment = SegmentDescriptor::new(0, 0xffffffff, 0x92, 0xC);
        encode_entry_at(0, SegmentDescriptor::NULL);
        encode_entry_at(1, code_segment);
        encode_entry_at(2, data_segment);

        let gdtr = TableDescriptor::new(&GDT);
        lgdt(&gdtr);

        gdt_flush();
        println!("GDT Initialized!");
        println!("{:?}\n{:?}", code_segment, data_segment);

        managment::print_memory_map(bootloader_info);
    }
}