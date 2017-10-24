arch ?= x86_64
LD := x86_64-elf-ld
LD_FLAGS := --oformat binary -b binary

kernel := build/kernel-$(arch).bin
assembly_source_files := src/arch/$(arch)/boot.asm #$(wildcard src/arch/$(arch)/*.s)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all run clean

all: $(kernel) run

$(kernel): $(assembly_object_files)
	@$(LD) $(LD_FLAGS) -o $@ $(assembly_object_files)

run: $(kernel)
	@qemu-system-$(arch) $<

clean:
	@rm -rf build

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(dir $@)
	@nasm -f bin -o $@ -i $(dir $<) $<

