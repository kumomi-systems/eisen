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

RT=$(realpath $(dirname $0)/..)

qemu-system-x86_64                                  \
  -L OVMF/                                          \
  -drive if=pflash,file=OVMF/OVMF.4m.fd,format=raw  \
  -drive format=raw,file=$RT/Eisen.img              \
  -net none                                         \
  -vga std                                          \
  -enable-kvm                                       \
	-cpu host                                         \
  -m 4G                                             \
  -debugcon stdio                                   \
  -d int                                            \
  -no-shutdown                                      \
  -no-reboot
