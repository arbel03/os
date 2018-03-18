use super::segmentation::SegmentSelector;

#[inline(always)]
pub unsafe fn load_cs(segment: SegmentSelector) {
    asm!("
    mov ax, $0
    jmp ax:.flush_cs
.flush_cs:
    " :: "m"(segment) : "ax" : "intel","volatile");
}

#[inline(always)]
pub unsafe fn load_ds(segment: SegmentSelector) {
    asm!("
    mov ax, $0
    mov ds, ax
    " :: "m"(segment) : "ax" : "intel","volatile");
}

#[inline(always)]
pub unsafe fn load_ss(segment: SegmentSelector) {
 asm!("
    mov ax, $0
    mov ss, ax
    " :: "m"(segment) : "ax" : "intel","volatile");
}