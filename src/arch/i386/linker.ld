ENTRY(header_start)

SECTIONS {
    .text : AT(0x7c00)
    {
        KEEP(*(.mbr_header))
        *(.text)
    }

    .data :
    {
        *(.rodata)
    }
}