arch ?= x86_64
LD := x86_64-elf-ld
LD_FLAGS := --oformat binary -b binary

kernel := build/kernel-$(arch).bin
linker_script := src/arch/$(arch)/linker.ld
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all run clean

all: $(kernel)

$(kernel): $(assembly_object_files) $(linker_script)
	$(LD) $(LD_FLAGS) -T $(linker_script) -o $@ $<

run: $(kernel)
	@qemu-system-$(arch) $<

clean:
	@rm -rf build

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	nasm -f bin -o $@ -i $< $<
