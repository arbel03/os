global header_start
extern protected_mode_start

section .mbr_header
bits 16
header_start:
	; Setting up a stack
	mov bp, 0x2000
	mov sp, bp

	mov bx, LOAD_DISK_MESSAGE
	call print_string

	mov bx, 0x9000
	mov dh, 1
	call disk_load

	; Switching to pmode
	call switch_to_pm

	jmp $
	
switch_to_pm:
	cli
	lgdt [gdt32.pointer]

	; Enter protected mode
	mov eax, cr0
	or eax, 1
	mov cr0, eax

	mov ax, gdt32.data
	jmp gdt32.code:protected_mode_start
	ret

print_string:
	mov ah, 0x0e
print_char:
	mov al, byte [bx]
	cmp al, 0
	je end
	int 0x10
	inc bx
	jmp print_char
end:
	ret

disk_load:
	push dx

	mov ah, 0x02 ; BIOS read sector function
	mov al, dh ; Read dh sectors
	mov ch, 0x00 ; Cylinder 0
	mov dh, 0x00 ; Select head 0
	mov cl, 0x02 ; Start reading from second sector (after mbr)
	int 0x13
	jc disk_error

	pop dx
	cmp dh, al ; if al (sectors read) != dh (sectors expected)
	jne disk_error
	ret
disk_error:
	mov bx, DISK_ERROR_MSG
	call print_string
	jmp $

; Variables
DISK_ERROR_MSG: db 'Disk read error.', 10, 13, 0
LOAD_DISK_MESSAGE: db "Loading sectors from disk.", 10, 13, 0

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

	; Padding
	times 510-($-$$) db 0
	dw 0xAA55
header_end: