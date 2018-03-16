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

    push esp

    call syscall_handler_inner
    
    pop esp

    pop edi
    pop esi
    pop edx
    pop ecx
    pop ebx
    ; Dont pop eax
    add esp, 4

    iretd

