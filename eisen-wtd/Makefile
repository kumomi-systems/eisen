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

HERE              := $(RT)/eisen-wtd
SRC               := $(HERE)/src
TARGET            := $(HERE)/target

DRIVER            := $(BIN)/eisen.efi

CARGO             := cargo
CARGO_BUILD_OPTS  := --artifact-dir $(BIN)/wakatiwai -Zunstable-options

.SILENT:
.PHONY: clean

all: driver

clean:
	rm -rf $(TARGET) $(KERNEL)

driver:
	$(CARGO) build $(CARGO_BUILD_OPTS)
	