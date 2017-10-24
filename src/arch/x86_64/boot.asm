global mbr_header
[org 0x7c00]

mbr_header:
	mov [BOOT_DRIVE], dl ; BIOS stores our boot drive in DL, storing it.

	;Setting up a stack
	mov bp, stack_bottom
	mov sp, bp

	mov bx, LOAD_DISK_MESSAGE
	call print_string

	mov bx, 0x9000 ; Where our code will be loaded
	mov dh, 1 ; Amout of sectors to read
	call disk_load

	jmp $

	%include "disk_load.asm"
	%include "print_string.asm"

	; Variables
	DISK_ERROR_MSG db "Disk read error!", 10, 13, 0
	LOAD_DISK_MESSAGE db "Loading sectors from disk", 10, 13, 0
	BOOT_DRIVE db 0

	times 510-($-$$) db 0
	dw 0xaa55

gdt32_start:
.null:
	dw 0x0
	dw 0x0
.code:
	;First flags- Preset(1), Privilege(00), Descriptor type(1)
	;Type flags- Code(1), Conforming(0), Readable(1), Accessed(0)
	;Second flags- Granulatiry(1), 32-bit(1), 64-bit(0), AVL(0)
	dw 0xffff ;Limit (bits 0-15)
	dw 0x0 ;Base (bits 0-15)
	db 0x0 ;Base (bits 16-23)
	db 10011010b ;First flags, Type flags
	db 11001111b ;Second flags, Limit(bits 16-19)
	db 0x0 ;Base (bits 24-31)
.data:
;Type flags- Code(0), Expand Down(0), Writeable(1), Accessed(0)
	dw 0xffff ;Limit (bits 0-15)
	dw 0x0 ;Base (bits 0-15)
	db 0x0 ;Base (bits 16-23)
	db 10010010b ;First flags, Type flags
	db 11001111b; Second flags, Limit(bits 16-19)
	db 0x0 ;Base (bits 24-31)
gdt32_end:

.pointer:
	dw gdt32_end - gdt32_start -1 ;Size
	dd gdt32_start ;Start address of the gdt

stack_top:
	resb 64
stack_bottom:
