/* Linker script for the STM32G474RE */
MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 512K
    RAM : ORIGIN = 0x20000000, LENGTH = 80K
    RAM2 : ORIGIN = 0x20014000, LENGTH = 16K
    CCMRAM : ORIGIN = 0x20018000, LENGTH = 32K
}

/* Put stack in CCMSRAM, should make it faster */
_stack_start = ORIGIN(CCMRAM) + LENGTH(CCMRAM);
