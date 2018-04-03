global context_switch

section .text
bits 32
context_switch:
    push 0b10111 ; User Stack
    push 1024*50 ; User Stack
    pushf
    push 0b00111 ; User Code
    push 0 ; User EIP
    iretd