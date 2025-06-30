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

.code16

.section .mbrtrap, "a"
mbr_trap_start:
  # Clear blinking cursor
  movw    $0x1003, %ax
  movw    $0x0000, %bx
  int     $0x10

  # Print trap message
  mov     mbr_trap_message, %si
  movb    $0x0E, %ah
  movb    $0x07, %bl
  .printstr:
    movb    (%si), %al
    cmpb    $0, %al
    je      .printstr_end
    int     $0x10
    inc     %si
    jmp     .printstr
  .printstr_end:

  # Actual trap
  jmp .

mbr_trap_message:
  .asciz "Eisen must be booted from 64-bit UEFI!"

.space 510-(.-mbr_trap_start)
.word 0xAA55
