.code64

.section .bootinfo, "adw"
.global bootinfo_start

.equ SECTION_SIZE,        0x40

.equ MAJOR_VERSION,       0
.equ MINOR_VERSION,       1
.equ PATCH_VERSION,       0

.equ VERSION_FLAG_ALPHA,  0b00000001

.equ VERSION_FLAGS, 0 | VERSION_FLAG_ALPHA

bootinfo_start:
  ## Product ID section
  .align  SECTION_SIZE
  .asciz  "Eisen"                         # Magic start number
  .align  0x10, 0x20                      # Reserved
  .space  0x10, 0x30                      # OS UUID (unique to each instance of Eisen)
  .byte   SECTION_SIZE                    # Section Size
  .byte   MAJOR_VERSION                   # Major Version
  .byte   MINOR_VERSION                   # Minor Version
  .byte   PATCH_VERSION                   # Patch Version
  .byte   VERSION_FLAGS                   # Version Flags
  .balign SECTION_SIZE, 0x20              # Reserved
  
  ## Kernel section
  .align  SECTION_SIZE
  .word   0x9459                          # Eisen UUID (bytes 0 and 1)
  .word   0x5C96                          # Eisen UUID (bytes 2 and 3)
  .space  4, 0x20                         # Reserved
  .quad   kentry                          # Address of kernel entry
  .space  8, 0x30                         # Address of kernel args
  .balign SECTION_SIZE, 0x20              # Reserved
  
  .align  SECTION_SIZE
  .word   0xBD12                          # Eisen UUID (bytes 4 and 5)
  .balign SECTION_SIZE, 0x20              # Reserved
  
  .align  SECTION_SIZE
  .word   0x40E1                          # Eisen UUID (bytes 6 and 7)
  .balign SECTION_SIZE, 0x20              # Reserved
  
  .align  SECTION_SIZE
  .word   0xA7FB                          # Eisen UUID (bytes 8 and 9)
  .balign SECTION_SIZE, 0x20              # Reserved

  .align  SECTION_SIZE
  .word   0x61C5                          # Eisen UUID (bytes 10 and 11)
  .word   0x0F3D                          # Eisen UUID (bytes 12 and 13)
  .word   0xCA9A                          # Eisen UUID (bytes 14 and 15)
  
  .space  480-(.-bootinfo_start), 0x20    # Reserved
  .long   0x10927C92                      # CRC32 checksum
  .space  504-(.-bootinfo_start), 0x20     
  .ascii  "InfoEnd"                       # Magic end number
  .byte   26                              # Magic end number
bootinfo_end:
