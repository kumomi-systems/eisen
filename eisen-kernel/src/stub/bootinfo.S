## Eisen Operating System
## Copyright (C) 2025  Kumomi Systems
## 
## This program is free software: you can redistribute it and/or modify
## it under the terms of the GNU General Public License as published by
## the Free Software Foundation, either version 3 of the License, or
## (at your option) any later version.
## 
## This program is distributed in the hope that it will be useful,
## but WITHOUT ANY WARRANTY; without even the implied warranty of
## MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
## GNU General Public License for more details.
## 
## You should have received a copy of the GNU General Public License
## along with this program. If not, see <https://www.gnu.org/licenses/>.

.code64

.section .bootinfo, "adw"
.global bootinfo_start

.equ SECTION_SIZE,        0x40
.equ SPACE,               0x00
.equ PLACEHOLDER,         0x30

.equ HEADER_VERSION,      0
.equ HEADER_SIZE,         0x200

.equ MAJOR_VERSION,       0
.equ MINOR_VERSION,       1
.equ PATCH_VERSION,       0

.equ VERSION_FLAG_ALPHA,  0b00000001  # Alpha release
.equ VERSION_FLAG_BETA,   0b00000010  # Beta release
.equ VERSION_FLAG_RC,     0b00000100  # Release candidate
.equ VERSION_FLAG_XPER,   0b00001000  # Experimental

.equ VERSION_FLAGS,       0 | VERSION_FLAG_ALPHA

.extern _kentry
.extern _kargs
.extern _ksysinfo
.extern STUB_END
.extern KERNEL_VMA
.extern KERNEL_SIZE
.extern STACK_TOP

bootinfo_start:
  ## Product ID section
  .align  SECTION_SIZE
  .asciz  "Eisen"                         # Magic start number
  .byte   HEADER_VERSION                  # Header version
  .word   HEADER_SIZE                     # Header size in bytes
  .align  0x10, SPACE                     # Reserved
  .space  0x10, PLACEHOLDER               # Product UUID (unique to each installation of Eisen)
  .byte   SECTION_SIZE                    # Section Size
  .align  0x04
  .space  0x2, PLACEHOLDER                # Year built
  .space  0x1, PLACEHOLDER                # Month built
  .space  0x1, PLACEHOLDER                # Day built
  .byte   MAJOR_VERSION                   # Major Version
  .byte   MINOR_VERSION                   # Minor Version
  .byte   PATCH_VERSION                   # Patch Version
  .byte   VERSION_FLAGS                   # Version Flags
  .align  0x10
  .asciz  "Hydrogen"                      # Version name
  .balign SECTION_SIZE, SPACE             # Reserved
  
  ## Kernel section
  .align  SECTION_SIZE
  .word   0x9459                          # Eisen UUID (bytes 0 and 1)
  .space  6, SPACE                        # Reserved
  .quad   _kentry                         # Address of kernel entry
  .quad   _kargs                          # Address of kernel args
  .quad   _ksysinfo                       # Address of system information
  .quad   STUB_END                        # End of stub / beginning of ELF header
  .quad   KERNEL_VMA                      # Kernel Virtual Memory Address 
  .quad   KERNEL_SIZE                     # Size of the kernel
  .quad   STACK_TOP                       # Top of the stack
  .balign SECTION_SIZE, SPACE             # Reserved
  
  .align  SECTION_SIZE
  .word   0x5C96                          # Eisen UUID (bytes 2 and 3)
  .balign SECTION_SIZE, SPACE             # Reserved
  
  .align  SECTION_SIZE
  .word   0xBD12                          # Eisen UUID (bytes 4 and 5)
  .balign SECTION_SIZE, SPACE             # Reserved
  
  .align  SECTION_SIZE
  .word   0x40E1                          # Eisen UUID (bytes 6 and 7)
  .balign SECTION_SIZE, SPACE             # Reserved

  .align  SECTION_SIZE
  .word   0xA7FB                          # Eisen UUID (bytes 8 and 9)
  .balign SECTION_SIZE, SPACE

  .align  SECTION_SIZE
  .word   0x61C5                          # Eisen UUID (bytes 10 and 11)
  .balign SECTION_SIZE, SPACE

  .align SECTION_SIZE
  .word   0x0F3D                          # Eisen UUID (bytes 12 and 13)
  
  .space  HEADER_SIZE-0x10-(.-bootinfo_start), SPACE
  .space  2, SPACE
  .word   0xCA9A                          # Eisen UUID (bytes 14 and 15)
  .space  4, 0x00                         # CRC32 checksum
  .ascii  "InfoEnd"                       # Magic end number
  .byte   26                              # Magic end number
bootinfo_end:
