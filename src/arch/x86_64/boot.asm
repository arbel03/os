[org 0x7c00]

	; Setting up a stack
	mov bp, 0x9000
	mov sp, bp

	; Switching to pmode
	call switch_to_pm

	jmp $

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

%include "switch_to_pm.asm"

[bits 32]
begin_pmode:
	; print 'OK' to screen
	mov dword [0xb8000], 0x2f4b2f4f
	jmp $
	ret

; Variables
DISK_ERROR_MSG db "Disk read error!", 10, 13, 0
LOAD_DISK_MESSAGE db "Loading sectors from disk", 10, 13, 0

; Padding
times 510-($-$$) db 0
dw 0xaa55
