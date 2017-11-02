global protected_mode_start

section .text
bits 32
protected_mode_start:
 	mov ds, ax
	mov ss, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	; Setting up a new stack
	mov ebp, 0x90000
	mov esp, ebp
    
    extern rust_main
    call rust_main

	; Print OK to the screen
	mov dword [0xb8000], 0x2f4b2f4f

    jmp $