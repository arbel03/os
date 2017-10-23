global mbr_header
[org 0x7c00]
mbr_header:
	mov [BOOT_DRIVE], dl ;BIOS stores our boot drive in DL, storing it.

	;Setting up a stack
	mov bp, 0x8000
	mov sp, bp

	mov bx, 0x9000 ;Where our code will be loaded
	mov dh, 2 ;Amout of sectors to read
	call disk_load

	jmp $

	disk_load:
		push dx

		mov ah, 0x02 ;BIOS read sector function
		mov al, dh ;Read dh sectors
		mov ch, 0x00 ;Cylinder 0
		mov dh, 0x00 ;Select head 0
		mov cl, 0x02 ;Start reading from second sector (after mbr)
		int 0x13
		jc disk_error

		pop dx
		cmp dh, al ;if al (sectors read) != dh (sectors expected)
		jne disk_error
		ret

	disk_error:
		mov bx, DISK_ERROR_MSG
		call print_string
		jmp $

	print_string:
		mov ah, 0x0e
	print_char:
		mov al, [bx]
		cmp al, 0
		je end
		mov ah, 0x0e
		int 0x10
		inc bx
		jmp print_char
	end:
		ret

	;Variables
	HELLO_MSG db 'PRINT THIS', 0
	DISK_ERROR_MSG db "Disk read error!", 0
	BOOT_DRIVE: db 0

	times 510-($-$$) db 0
	dw 0xaa55

;Filling 2 sectors with junk
times 512 dw 'A'

