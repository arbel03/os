use dtables::Segmentation::*;
use dtables::lgdt;
use dtables::TableDescriptor;

extern {
    fn gdt_flush();
}

// My gdt, containing 8 bytes entries (or unsigned 64 bit values)
static mut GDT: [u64; 5] = [0; 5];

pub unsafe fn encodeEntryAt(index: usize, entry: SegmentDescriptor) {
    let mut descriptor_high: u32 = 0;
    // Create the high 32 bit segment
    descriptor_high  =  (entry.base & 0x00FF0000) >> 16;
    descriptor_high |= (entry.access_byte as u32) << 8;
    descriptor_high |= (entry.limit & 0x000F0000);
    descriptor_high |=  (entry.flags as u32) << 20;
    descriptor_high |= entry.base & 0xFF000000;        

    let mut descriptor_low: u32 = 0;
    // Create the low 32 bit segment
    descriptor_low |= entry.base << 16;                 // set base bits 15:0
    descriptor_low |= entry.limit & 0xFFFF;               // set limit bits 15:0

    let mut descriptor: u64 = 0;
    descriptor = (descriptor_high as u64) << 32;
    descriptor |= descriptor_low as u64;

    GDT[index] = descriptor;
}

pub fn init() {
    unsafe {
        let codeSegment = SegmentDescriptor::new(0, 0xffffffff, 0x9A, 0xC);
        let dataSegment = SegmentDescriptor::new(0, 0xffffffff, 0x92, 0xC);
        encodeEntryAt(0, SegmentDescriptor::NULL);
        encodeEntryAt(1, codeSegment);
        encodeEntryAt(2, dataSegment);

        let GDTR = TableDescriptor::new(&GDT);
        lgdt(&GDTR);

        gdt_flush();
        println!("GDT Initialized!");
        println!("{:?}\n{:?}", codeSegment, dataSegment);
    }
}