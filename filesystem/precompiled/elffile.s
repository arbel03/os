global _start

segment .text
_start: 
    push message
    jmp $

segment .data 
    message db 'Hello World',0
