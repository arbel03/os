ENTRY(start)

SECTIONS {
    . = 1M;

    .text : {
        *(.text)
    }
}