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