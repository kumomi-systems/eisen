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

OUTPUT_FORMAT(elf64-x86-64)
ENTRY(_kentry)

KERNEL_VMA        = 0x10000000;

SECTIONS
{
  .             = 0;
  .mbrtrap                : {
    *(.mbrtrap)
  }
  .bootinfo ALIGN(0x200)  : {
    *(.bootinfo)
  }
  STUB_END      = .;

  .             = KERNEL_VMA;
  .             += STUB_END;
  KERNEL_START  = .;

  .kentry   ALIGN(4K)     : AT (ADDR (.kentry) - KERNEL_VMA) {
    *(.kentry)
  }
  .kdata    ALIGN(4K)     : AT (ADDR (.kdata)  - KERNEL_VMA) {
    *(.kdata)
  }
  .text     ALIGN(4K)     : AT (ADDR (.text)   - KERNEL_VMA) {
    *(.text)
    *(.text.*)
  }
  .rodata   ALIGN(4K)     : AT (ADDR (.rodata) - KERNEL_VMA) {
    *(.rodata)
    *(.rodata.*)
  }
  .data     ALIGN(4K)     : AT (ADDR (.data)   - KERNEL_VMA) {
    *(.data)
    *(.data.*)
  }
  .bss      ALIGN(4K)     : AT (ADDR (.bss)    - KERNEL_VMA) {
    *(COMMON)
    *(.bss)
  }

  KERNEL_SIZE   = . - KERNEL_START;

  .late_discard : {
    *(.got)
    *(.got.*)
  }

  /DISCARD/ : { *(*) }
}