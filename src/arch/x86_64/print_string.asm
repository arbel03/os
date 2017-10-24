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
