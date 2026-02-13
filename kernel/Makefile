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

HERE              := $(RT)/kernel
SRC               := $(HERE)/src
TARGET            := $(HERE)/target

STUB							:= $(BIN)/stub.bin
KERNEL_ELF        := $(BIN)/kernel.elf
KERNEL_BIN        := $(BIN)/kernel.bin
KERNEL						:= $(BIN)/kernel

CARGO             := cargo build
CARGO_BUILD_OPTS  := --artifact-dir $(BIN) -Zunstable-options

AS                := as
AS_OPTS           := 

LD                := ld
LD_OPTS           := --script=linker.ld -static -Bsymbolic -nostdlib --output binary

.SILENT:
.PHONY: clean

all: kernel-rust
	bash -c "$(HERE)/gen-env-linker.sh"
	$(LD) $(LD_OPTS) $(shell find $(BIN) -name "*.[oa]") -o $(KERNEL_BIN)
	objdump -xdsZ $(KERNEL_BIN) > $(KERNEL_BIN).dis
	bash -c "$(HERE)/strip.sh $(STUB) $(KERNEL_BIN)"
	bash -c "$(HERE)/sign.sh $(STUB)"
	cat $(STUB)				>  $(KERNEL)
	cat $(KERNEL_BIN) >> $(KERNEL)

clean:
	rm -rf $(TARGET)

kernel-asm: $(shell find $(SRC) -name "*.S")
	$(AS) $(AS_OPTS) $^ -o $(BIN)/kernel_asm.o

kernel-rust: kernel-asm
	$(CARGO) $(CARGO_BUILD_OPTS)
	