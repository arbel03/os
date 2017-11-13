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
