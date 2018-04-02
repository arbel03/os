global context_switch

section .text
bits 32
context_switch:
    mov ax, 0b10111
    mov ds, ax

    push 0b11111
    push 1024*50
    pushf
    push 0b01111
    push 0
    iretd
continue_context:
    ; jmp $
    ret