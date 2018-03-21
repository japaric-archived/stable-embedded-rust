INCLUDE memory.x;

/* Entry point = reset handler */
ENTRY(reset);

/* Required symbols */
/* (stable workaround for `#[used]`) */
EXTERN(default_handler);
EXTERN(RESET_VECTOR);
EXTERN(EXCEPTIONS);

/* Weakly alias the exception handlers to the default exception handler */
/* (stable workaround for `#[linkage = "weak"]`) */
PROVIDE(nmi = default_handler);
PROVIDE(hard_fault = default_handler);
PROVIDE(mem_manage = default_handler);
PROVIDE(bus_fault = default_handler);
PROVIDE(usage_fault = default_handler);
PROVIDE(svcall = default_handler);
PROVIDE(pendsv = default_handler);
PROVIDE(systick = default_handler);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    LONG(ORIGIN(RAM) + LENGTH(RAM)); /* Initial Stack Pointer value */
    KEEP(*(.vector_table.reset_vector));
    __reset_vector = .;
    KEEP(*(.vector_table.exceptions));
    __exceptions = .;
  } > FLASH

  .text :
  {
    *(.text .text.*);
  } > FLASH

  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

  .data :
  {
    _sidata = LOADADDR(.data);

    . = ALIGN(4);
    _sdata = .;

    *(.data .data.*);

    . = ALIGN(4);
    _edata = .;
  } > RAM AT > FLASH

  .bss :
  {
    . = ALIGN(4);
    _sbss = .;

    *(.bss .bss.*);

    . = ALIGN(4);
    _ebss = .;
  } > RAM

  /DISCARD/ :
  {
    *(.ARM.exidx.*)
  }
}

/* Check the memory layout of the vector table */
ASSERT(__reset_vector == ORIGIN(FLASH) + 0x8, "Reset vector is missing");
ASSERT(__exceptions == ORIGIN(FLASH) + 0x40, "Exception vectors are missing");
