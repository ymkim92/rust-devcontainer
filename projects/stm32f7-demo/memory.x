MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 2048K
  RAM   : ORIGIN = 0x20000000, LENGTH = 512K
}

/* Define the entry point */
ENTRY(_start)

/* The following sections define the memory regions for the program code and data */
SECTIONS
{
    /* Program code and read-only data goes into Flash */
    .text : 
    {
        KEEP(*(.isr_vector)) /* Interrupt vectors */
        *(.text*)            /* Application code */
        *(.rodata*)          /* Read-only data */
        KEEP(*(.init*))      /* Initialization code */
        KEEP(*(.fini*))      /* Finalization code */
    } > FLASH

    /* Data segment, initialized data goes into RAM */
    .data : AT(ADDR(.text) + SIZEOF(.text))
    {
        *(.data*)            /* Initialized data */
    } > RAM

    /* BSS section for uninitialized data */
    .bss :
    {
        *(.bss*)
    } > RAM

    /* Stack space */
    _stack_start = ORIGIN(RAM) + LENGTH(RAM);
    _stack_end = _stack_start - 0x1000; /* 4KB stack */

    PROVIDE(_heap_start = _stack_end);
    PROVIDE(_heap_end = ORIGIN(RAM));
}