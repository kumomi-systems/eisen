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

clean:
	rm -rf $(TARGET) $(KERNEL)

stub: $(shell find $(SRC)/stub -name "*.S")
	$(AS) $(AS_OPTS) $^ -o $(BIN)/stub.o

kernel:
	$(CARGO) build $(CARGO_BUILD_OPTS)
	