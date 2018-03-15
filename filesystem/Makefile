bin_target := i686-unknown-linux-musl
bin_names := hello1 hello2
bin_files := $(patsubst %, build/isofiles/bin/%, $(bin_names))

.PHONY: filesystem

filesystem: xargo-bin $(bin_files)

xargo-bin: 
	@cd filesystem; cargo build --target=$(bin_target) --bins -v

clean-filesystem:
	@cargo clean
	@rm -r filesystem/target/

build/isofiles/bin/%: filesystem/target/$(bin_target)/debug/%
	@mkdir -p build/isofiles/bin
	@cp $< $@