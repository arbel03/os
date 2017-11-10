global start

section .text
bits 32
start:
	lgdt [gdt32.pointer]
	call gdt32.code:protected_mode_start
	hlt

protected_mode_start:
	mov ax, gdt32.data
	mov ss, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	; Loading stack at the end of memory
	mov esp, 0xffff
	mov ebp, esp

	; Call the rust kernel
	extern rust_main
	call rust_main
	ret

section .rodata
gdt32:
	dd 0x0
	dd 0x0
.code: equ $-gdt32
	;First flags- Preset(1), Privilege(00), Descriptor type(1)
	;Type flags- Code(1), Conforming(0), Readable(1), Accessed(0)
	;Second flags- Granulatiry(1), 32-bit(1), 64-bit(0), AVL(0)
	dw 0xffff ;Limit (bits 0-15)
	dw 0x0 ;Base (bits 0-15)
	db 0x0 ;Base (bits 16-23)
	db 10011010b ;First flags, Type flags
	db 11001111b ;Second flags, Limit(bits 16-19)
	db 0x0 ;Base (bits 24-31)
.data: equ $-gdt32
	;Type flags- Code(0), Expand Down(0), Writeable(1), Accessed(0)
	dw 0xffff ;Limit (bits 0-15)
	dw 0x0 ;Base (bits 0-15)
	db 0x0 ;Base (bits 16-23)
	db 10010010b ;First flags, Type flags
	db 11001111b; Second flags, Limit(bits 16-19)
	db 0x0 ;Base (bits 24-31)
.pointer:
	dw $ - gdt32 - 1 ;Size
	dd gdt32 ;Start address of the gdt
