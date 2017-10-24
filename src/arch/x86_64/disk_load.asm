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
