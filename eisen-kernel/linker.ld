OUTPUT_FORMAT(binary)
ENTRY(kentry)

SECTIONS
{
  .               = 0;
  IMG_START       = .;
  STUB_START      = .;
  .mbrtrap        : {
    *(.mbrtrap)
  }
  .bootinfo       : ALIGN(0x200) {
    *(.bootinfo)
  }
  STUB_END        = .;

  .               = 1M;
  KERNEL_START    = .;
  .text           : ALIGN(0x1000) {
    *(.text)
    *(.text.*)
  }
  .data           : ALIGN(0x1000) {
    *(.data)
  }
  .bss            : ALIGN(0x1000) {
    *(.bss)
  }
  .kargs          : ALIGN(0x1000) {
    *(.kargs)
  }
  KERNEL_END      = .;
  
  /DISCARD/ : { *(*) }
}