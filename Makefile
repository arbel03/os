arch ?= i386
target ?= $(arch)-sos

linker_script := src/arch/$(arch)/linker.ld

rust_os := target/$(target)/debug/libsos.a

assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

kernel := build/kernel-$(arch).bin
iso := build/os-$(arch).iso
grub_cfg := src/arch/$(arch)/grub.cfg

.PHONY: all clean run iso kernel debug

all: $(kernel) run
 
$(kernel): kernel $(rust_os) $(assembly_object_files) $(linker_script)
	@i386-elf-ld -n --gc-sections -m elf_i386 -T $(linker_script) -o $@ $(assembly_object_files) $(rust_os)

kernel:
	@xargo build --target $(target)

run: $(iso)
	@qemu-system-i386 -cdrom $(iso) -s

debug: $(iso)
	@qemu-system-i386 -cdrom $(iso) -s -S

gdb:
	gdb "build/kernel-$(arch).bin" -ex "target remote :1234" 

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) build/isofiles 2> /dev/null

clean:
	@rm -rf build
	@rm -rf target

build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(dir $@)
	@nasm -f elf32 -o $@ $<
