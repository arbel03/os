nasm -f elf32 elffile.s -o elffile.o
i686-elf-ld -o elffile -Ttext=0 elffile.o