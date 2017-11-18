global start

section .text
bits 32
start:
	extern kmain
	call kmain
	hlt
