rust_os := target/i386-sos/debug/libsos.a

assembly_source_files := $(wildcard src/arch/*.asm)
assembly_object_files := $(patsubst src/arch/%.asm, \
	build/arch/%.o, $(assembly_source_files))

iso := build/os.iso
kernel := build/kernel.bin

.PHONY: clean all run cargo

all: run

run: $(iso)
	@qemu-system-i386 -drive file=$<,format=raw

$(kernel): cargo $(rust_os) $(assembly_object_files)
	@i386-elf-ld -n --gc-sections -m elf_i386 -T linker.ld -o $@ $(assembly_object_files) $(rust_os)

cargo:
	@xargo build --target i386-sos

clean:
	@rm -rf build

build/arch/%.o: src/arch/%.asm
	@mkdir -p $(dir $@)
	@nasm -f elf32 -o $@ $<

include bootloader/Makefile
