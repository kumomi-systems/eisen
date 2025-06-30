OUTPUT_FORMAT(binary)
ENTRY(_kentry)

KERNEL_LOAD_ADDR  = 0xA0000000;
KERNEL_OFFSET     = 1M;

SECTIONS
{
  .               = 0;
  IMG_START       = .;
  STUB_START      = .;
  .mbrtrap        : LENGTH = 0x200 {
    *(.mbrtrap)
  }
  .bootinfo       : ALIGN(0x200), LENGTH = 0x200 {
    *(.bootinfo)
  }
  STUB_END        = .;

  .               = KERNEL_LOAD_ADDR;
  KERNEL_START    = .;
  .               += KERNEL_OFFSET;
  .text   ALIGN(0x1000) : AT (ADDR (.text)  - KERNEL_LOAD_ADDR) {
    *(.text)
    *(.text.*)
  }
  .data   ALIGN(0x1000) : AT (ADDR (.data)  - KERNEL_LOAD_ADDR) {
    *(.data)
  }
  .bss    ALIGN(0x1000) : AT (ADDR (.bss)   - KERNEL_LOAD_ADDR) {
    *(.bss)
  }
  .kdata  ALIGN(0x1000) : AT (ADDR (.kdata) - KERNEL_LOAD_ADDR) {
    *(.kdata)
  }
  KERNEL_SIZE     = . - KERNEL_START + 1;
  
  /DISCARD/ : { *(*) }
}