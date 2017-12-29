#[repr(packed, C)]
#[derive(Clone, Copy, Debug)]
pub struct SegmentDescriptor {
    pub base: u32,
    pub limit: u32,
    pub access_byte: u8,
    pub flags: u8
}

impl SegmentDescriptor {
    pub const NULL: SegmentDescriptor = SegmentDescriptor { 
        base: 0, 
        limit: 0, 
        access_byte: 0, 
        flags: 0 
    };

    pub fn new(base: u32, limit: u32, access_byte: u8, flags: u8) -> SegmentDescriptor {
        SegmentDescriptor { base, limit, access_byte, flags }
    }
}