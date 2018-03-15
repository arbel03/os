bin_target := i686-unknown-linux-musl
bin_names := hello
debug_bin_files := $(patsubst %, filesystem/target/$(bin_target)/debug/%, $(bin_names))

.PHONY: filesystem xargo-bin

filesystem: xargo-bin move-bins

xargo-bin: 
	@cd filesystem; cargo build --target=$(bin_target) --bin hello

move-bins: $(debug_bin_files)
	@echo "Copying $< to build/isofiles"
	@mkdir -p build/isofiles
	@cp $< build/isofiles/bin

clean-filesystem:
	@cargo clean
	@rm -rf filesystem/target