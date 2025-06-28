export RT   := $(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
export BIN  := $(RT)/bin

.SILENT:
.PHONY: clean kernel wakatiwai-driver

all: lib kernel wtd

clean:
	rm -rf $(BIN)
	make -C $(RT)/eisen-lib clean
	make -C $(RT)/eisen-kernel clean
	make -C $(RT)/eisen-wtd clean

prebuild:
	mkdir -p $(BIN)

lib: prebuild
	make -C $(RT)/eisen-lib

kernel: prebuild
	make -C $(RT)/eisen-kernel

wtd: prebuild
	make -C $(RT)/eisen-wtd