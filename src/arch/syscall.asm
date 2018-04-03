global syscall_handler
extern syscall_handler_inner

section .text
bits 32
syscall_handler:
    push eax
    push ebx
    push ecx
    push edx
    push esi
    push edi

    mov eax, esp

    ; Data segments
    push gs
    push fs
    push es
    push ds

    push eax

    call syscall_handler_inner
    
    add esp, 4

    ; Data segments
    pop ds
    pop es
    pop fs
    pop gs

    pop edi
    pop esi
    pop edx
    pop ecx
    pop ebx
    ; Dont pop eax
    add esp, 4

    iretd

