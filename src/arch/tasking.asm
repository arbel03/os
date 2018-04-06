global context_switch

section .text
bits 32
context_switch:
    push esi ; User Stack Segment
    push eax
    pushf
    push edx ; User Code Segment
    push 0 ; User EIP
    iretd