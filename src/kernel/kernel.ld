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

    PROVIDE(_stack_start = bss_end);
    /* Stack size of 4kB */
    PROVIDE(_stack_end = _stack_start + 0x4000);

    PROVIDE(_HEAP_START = _stack_end);
    PROVIDE(_PHYSTOP = 0x88000000);
}
