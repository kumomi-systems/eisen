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

KERNEL            := $(BIN)/kernel.bin

CARGO             := cargo
CARGO_BUILD_OPTS  := --artifact-dir $(BIN) -Zunstable-options

AS                := as
AS_OPTS           := 

LD                := ld
LD_OPTS           := --script=linker.ld

.SILENT:
.PHONY: clean

all: stub kernel
	$(LD) $(LD_OPTS) $(shell find $(BIN) -name "*.[oa]") -o $(KERNEL)
	$(HERE)/sign.sh $(KERNEL)

clean:
	rm -rf $(TARGET) $(KERNEL)

stub: $(shell find $(SRC)/stub -name "*.S")
	$(AS) $(AS_OPTS) $^ -o $(BIN)/stub.o

kernel:
	$(CARGO) build $(CARGO_BUILD_OPTS)
	