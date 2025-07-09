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

export RT   := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
export BIN  := $(RT)/bin

.SILENT:
.PHONY: clean kernel wakatiwai-driver

all: fresh lib kernel wtd
	objdump -xDS $(BIN)/kernel.elf > $(BIN)/kernel.elf.dis

clean:
	rm -rf $(BIN)
	make -C $(RT)/eisen-lib clean
	make -C $(RT)/eisen-kernel clean
	make -C $(RT)/eisen-wtd clean

fresh:
	rm -rf $(BIN)

prebuild:
	mkdir -p $(BIN)

lib: prebuild
	make -C $(RT)/eisen-lib

kernel: prebuild
	make -C $(RT)/eisen-kernel

wtd: prebuild
	make -C $(RT)/eisen-wtd