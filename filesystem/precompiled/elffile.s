global _start

segment .text
_start:
    mov eax, 0x11
    mov ebx, message.contents
    mov ecx, message.length
    int 0x80
    jmp $

segment .data 
    message: 
        .contents: db 'bin/hello1'
        .length: equ 10
