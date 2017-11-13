global start

section .text
bits 32
start:
	extern rust_main
	call rust_main
	hlt
