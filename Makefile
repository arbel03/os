arch ?= x86_64
LD := x86_64-elf-ld
LD_FLAGS := --oformat binary -b binary
kernel := build/kernel-$(arch).bin
assembly_source_files := $(wildcard src/arch/$(arch)/*.s)
assembly_object_files := $(patsubst src/arch/$(arch)/%.s, build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all

all: $(kernel)

$(kernel): $(assembly_object_files)
	$(LD) $(LD_FLAGS) -o $@ $<

build/arch/$(arch)/%.o: src/arch/$(arch)/%.s
	mkdir -p $(shell dirname $@)
	nasm -f bin $< -o $@
