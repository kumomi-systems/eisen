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

HERE              := $(RT)/eisen-kernel
SRC               := $(HERE)/src
TARGET            := $(HERE)/target

STUB							:= $(BIN)/stub.bin
KERNEL_ELF        := $(BIN)/kernel.elf
KERNEL_BIN        := $(BIN)/kernel.bin

CARGO             := cargo build
CARGO_BUILD_OPTS  := --artifact-dir $(BIN) -Zunstable-options

AS                := as
AS_OPTS           := 

LD                := ld
LD_OPTS           := --script=linker.ld -static -Bsymbolic -nostdlib

.SILENT:
.PHONY: clean

all: kernel-asm kernel-rust
	$(LD) $(LD_OPTS) $(shell find $(BIN) -name "*.[oa]") -o $(KERNEL_ELF)
	$(HERE)/strip.sh $(STUB) $(KERNEL_ELF)
	$(HERE)/sign.sh $(STUB)
	cat $(STUB)				>  $(KERNEL_BIN)
	cat $(KERNEL_ELF) >> $(KERNEL_BIN)

clean:
	rm -rf $(TARGET)

kernel-asm: $(shell find $(SRC) -name "*.S")
	$(AS) $(AS_OPTS) $^ -o $(BIN)/kernel_asm.o

kernel-rust:
	$(CARGO) $(CARGO_BUILD_OPTS)
	