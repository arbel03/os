// section .rodata
// gdt32:
// 	dd 0x0
// 	dd 0x0
// .code: equ $-gdt32
// 	;First flags- Preset(1), Privilege(00), Descriptor type(1)
// 	;Type flags- Code(1), Conforming(0), Readable(1), Accessed(0)
// 	;Second flags- Granulatiry(1), 32-bit(1), 64-bit(0), AVL(0)
// 	dw 0xffff ;Limit (bits 0-15)
// 	dw 0x0 ;Base (bits 0-15)
// 	db 0x0 ;Base (bits 16-23)
// 	db 10011010b ;First flags, Type flags
// 	db 11001111b ;Second flags, Limit(bits 16-19)
// 	db 0x0 ;Base (bits 24-31)
// .data: equ $-gdt32
// 	;Type flags- Code(0), Expand Down(0), Writeable(1), Accessed(0)
// 	dw 0xffff ;Limit (bits 0-15)
// 	dw 0x0 ;Base (bits 0-15)
// 	db 0x0 ;Base (bits 16-23)
// 	db 10010010b ;First flags, Type flags
// 	db 11001111b; Second flags, Limit(bits 16-19)
// 	db 0x0 ;Base (bits 24-31)
// .pointer:
// 	dw $ - gdt32 - 1 ;Size
// 	dd gdt32 ;Start address of the gdt

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GDTDescriptor(u64);

pub fn create_descriptor(base: u32, limit: u32, flags: u16) -> GDTDescriptor {
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
    return GDTDescriptor(descriptor);
}

#[repr(C)]
struct GDTPointer(u16, u32);

#[repr(C)]
struct GDT {
    descriptors: &'static [GDTDescriptor],
    pointer: GDTPointer,
}