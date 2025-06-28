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
	