#[inline(always)]
pub unsafe fn load_cs(segment: u32) {
    asm!("
    mov ax, $0
    jmp ax:.flush_cs
.flush_cs:
    " :: "m"(segment) : "ax" : "intel","volatile");
}

#[inline(always)]
pub unsafe fn load_ds(segment: u32) {
    asm!("
    mov ax, $0
    mov ds, ax
    " :: "m"(segment) : "ax" : "intel","volatile");
}

#[inline(always)]
pub unsafe fn load_ss(segment: u32) {
 asm!("
    mov ax, $0
    mov ss, ax
    " :: "m"(segment) : "ax" : "intel","volatile");
}

#[inline(always)]
pub unsafe fn load_es(segment: u32) {
 asm!("
    mov ax, $0
    mov es, ax
    " :: "m"(segment) : "ax" : "intel","volatile");
}

#[inline(always)]
pub unsafe fn load_gs(segment: u32) {
 asm!("
    mov ax, $0
    mov gs, ax
    " :: "m"(segment) : "ax" : "intel","volatile");
}

#[inline(always)]
pub unsafe fn load_fs(segment: u32) {
 asm!("
    mov ax, $0
    mov fs, ax
    " :: "m"(segment) : "ax" : "intel","volatile");
}