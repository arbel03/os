[bits 16]
switch_to_pm:
 	cli
	lgdt [gdt32.pointer]

	; Enter protected mode
	mov eax, cr0
	or eax, 1
	mov cr0, eax

	jmp gdt32.code:init_pm

[bits 32]
init_pm:
 	mov ax, gdt32.data
 	mov ds, ax
	mov ss, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	; Setting up a new stack
	mov ebp, 0x90000
	mov esp, ebp
	; Switching to protected mode code
	call begin_pmode
