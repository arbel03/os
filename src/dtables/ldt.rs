#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SegmentDescriptor(pub u64);

impl SegmentDescriptor {
    pub fn new(base: u32, limit: u32, flags: u16) -> SegmentDescriptor {
        let mut descriptor_high: u32 = 0;
        // Create the high 32 bit segment
        descriptor_high  =  limit & 0x000F0000;         // set limit bits 19:16
        descriptor_high |= ((flags <<  8) as u32) & 0x00F0FF00;         // set type, p, dpl, s, g, d/b, l and avl fields
        descriptor_high |= (base >> 16) & 0x000000FF;         // set base bits 23:16
        descriptor_high |=  base & 0xFF000000;         // set base bits 31:24

        let mut descriptor_low: u32 = 0;
        // Create the low 32 bit segment
        descriptor_low |= base  << 16;                       // set base bits 15:0
        descriptor_low |= limit  & 0x0000FFFF;               // set limit bits 15:0

        let mut descriptor: u64 = 0;
        descriptor = (descriptor_high as u64) << 32;
        descriptor |= descriptor_low as u64;
        return SegmentDescriptor(descriptor);
    }
}