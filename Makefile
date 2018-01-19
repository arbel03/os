target ?= target
rust_os := target/$(target)/debug/libsos.a

assembly_source_files := $(wildcard src/arch/*.asm)
assembly_object_files := $(patsubst src/arch/%.asm, \
	build/arch/%.o, $(assembly_source_files))

filesystem := build/filesystem.bin
iso := build/os.iso
filesystem_head := build/os_head.bin
kernel := build/kernel.bin

.PHONY: clean all run cargo $(filesystem)

all: run

run: $(iso)
	@qemu-system-i386 -drive file=$(iso),format=raw

$(iso): $(filesystem_head) $(filesystem)
	@cat $(filesystem_head) > $@
	@dd if=$(filesystem) of=$@ count=90 bs=1 conv=notrunc
	@dd if=$(filesystem) skip=$(shell echo $$(( $(shell stat -L -c %s $(filesystem_head)) / 512 )) ) bs=512 >> $@

$(filesystem): $(filesystem_head)
	@dd if=/dev/zero of=$@ bs=1M count=50
	-@sudo umount /mnt/tmp || /bin/true
	@/sbin/mkfs.msdos -F 32 -R $(shell echo $$(( $(shell stat -L -c %s $(filesystem_head)) / 512)) ) $@

	@mkdir -p build/isofiles/dir
	@echo 'This is a sample file for testing' > build/isofiles/dir/file.txt

	@sudo mount -o loop $@ /mnt/tmp
	@sudo cp -r build/isofiles/. /mnt/tmp
	-@sudo umount /mnt/tmp || /bin/true
	@rm -r build/isofiles

$(filesystem_head): $(kernel)
	@mkdir -p build
	@nasm -f bin -o $@ -i bootloader/ bootloader/src/bootloader.asm

$(kernel): cargo $(rust_os) $(assembly_object_files)
	@ld -n --gc-sections -m elf_i386 -T linker.ld -o $@ $(assembly_object_files) $(rust_os);\

cargo:
	@export RUST_TARGET_PATH=$(shell pwd); xargo build --target $(target)

clean:
	@rm -rf build
	@xargo clean

build/arch/%.o: src/arch/%.asm
	@mkdir -p $(dir $@)
	@nasm -f elf32 -o $@ $<