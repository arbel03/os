target ?= target
rust_os := target/$(target)/debug/libsos.a

assembly_source_files := $(wildcard src/arch/*.asm)
assembly_object_files := $(patsubst src/arch/%.asm, build/arch/%.o, $(assembly_source_files))

filesystem_head := build/head.bin
kernel := build/kernel.bin

.PHONY: clean all run cargo head

head: $(kernel)
	@mkdir -p build
	@nasm -f bin -o build/head.bin -i bootloader/ bootloader/src/bootloader.asm

$(kernel): cargo $(rust_os) $(assembly_object_files)
	@$(LD) -n --gc-sections -m elf_i386 -T linker.ld -o $@ $(assembly_object_files) $(rust_os)

cargo:
	@export RUST_TARGET_PATH=$(shell pwd); xargo build --target $(target)

clean:
	@rm -rf build
	@xargo clean

build/arch/%.o: src/arch/%.asm
	@mkdir -p $(dir $@)
	@nasm -f elf32 -o $@ $<