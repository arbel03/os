bin_target := i686-unknown-linux-musl
bin_names := hello1 hello2
bin_files := $(patsubst %, filesystem/target/$(bin_target)/debug/%, $(bin_names))

.PHONY: filesystem

filesystem: xargo-bin copy-bins

xargo-bin:
	@cd filesystem; cargo build --target=$(bin_target) --bins

clean-filesystem:
	@cargo clean
	@rm -r filesystem/target/

copy-bins:
	@mkdir -p build/isofiles/bin
	@cp $(bin_files) build/isofiles/bin
	@cp -r filesystem/precompiled/ build/isofiles/bin