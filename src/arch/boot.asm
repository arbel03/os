global start

section .text
bits 32
start:
	; Setting up a stack
	mov esp, 0x90000
	mov ebp, esp

	push edi
	extern kmain
	call kmain
	hlt