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

#!/usr/bin/env bash

STUB=$1
KERNEL_ELF=$2

echo "Stripping kernel stub $STUB ..."

# Extract binary stub
objcopy                 \
  -j .mbrtrap           \
  -j .bootinfo          \
  -I elf64-x86-64       \
  -O binary             \
  $KERNEL_ELF $STUB

# Remove unwanted sections
objcopy                 \
  -R .mbrtrap           \
  -R .bootinfo          \
  -R .late_discard      \
  -F elf64-x86-64       \
  $KERNEL_ELF