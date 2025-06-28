HERE              := $(RT)/eisen-lib
SRC               := $(HERE)/src
TARGET            := $(HERE)/target

LIB               := $(BIN)/eisenlib

CARGO             := cargo
CARGO_BUILD_OPTS  := --artifact-dir $(BIN) -Zunstable-options

.SILENT:
.PHONY: clean

all: lib

clean:
	rm -rf $(TARGET) $(KERNEL)

lib:
	$(CARGO) build $(CARGO_BUILD_OPTS)
	