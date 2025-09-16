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

KERNEL_OFFSET   = 0xFFFFFFFF80000000;

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

  .             = ALIGN(4K);
  KERNEL_LMA    = .;

  .             = KERNEL_OFFSET;
  KERNEL_VMA    = .;
  KERNEL_START  = .;

  .kentry   ALIGN(4K)     : AT (KERNEL_LMA) {
    *(.kentry)
  }
  .kdata    ALIGN(4K)     : {
    *(.kdata)
  }
  .text     ALIGN(4K)     : {
    *(.text)
    *(.text.*)
  }
  .rodata   ALIGN(4K)     : {
    *(.rodata)
    *(.rodata.*)
  }
  .data     ALIGN(4K)     : {
    *(.data)
    *(.data.*)
  }
  .bss      ALIGN(4K)     : {
    *(.bss)
    *(.bss.*)
    *(.paging)
    *(COMMON)
  }
  KERNEL_SIZE   = . - KERNEL_START;

  .late_discard : {
    *(.got)
    *(.got.*)
  }

  /DISCARD/ : { *(*) }
}