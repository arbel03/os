arch ?= i386
target ?= $(arch)-sos
LD := $(arch)-elf-ld
linker_config := src/arch/$(arch)/linker.ld
LD_FLAGS := -n --oformat binary -T $(linker_config) -Ttext 0x7c00
kernel := build/kernel-$(arch).bin
rust_os := target/$(target)/debug/libsos.a
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm,build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run

all: $(kernel) run

$(kernel): kernel $(rust_os) $(assembly_object_files)
	@$(LD) $(LD_FLAGS) -o $@ $(assembly_object_files) $(rust_os)

kernel:
	@xargo build --target $(target)

run: $(kernel)
	@qemu-system-$(arch) -drive format=raw,file=$(kernel)

clean:
	@rm -rf build

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(dir $@)
	@nasm -f elf32 -o $@ -i $(dir $<) $<