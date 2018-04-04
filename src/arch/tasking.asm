global context_switch

section .text
bits 32
context_switch:
    push esi ; User Stack
    push eax
    pushf
    push edx ; User Code
    push 0 ; User EIP
    iretd