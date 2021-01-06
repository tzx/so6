OUTPUT_ARCH("riscv")

ENTRY(_start)

SECTIONS
{
    . = 0x80000000;

    .text :
    {
        *(.text.init)
        *(.text*)
    }

    .rodata ALIGN(0x1000) :
    {
        PROVIDE(_global_pointer = .);
        *(.rodata*)
    }

    .data ALIGN(0x1000) :
    {
        *(.sdata*)
        *(.data*)
    }

    .bss ALIGN(0x1000) :
    {
        bss_start = .;
        *(.sbss*)
        *(.bss*)
        bss_end = .;
    }

    /* Set stack to the top of RAM at 128M */
    PROVIDE(_stack = 0x88000000);
}
