bin_target := i686-bin
bin_names := shell
bin_files := $(patsubst %, filesystem/target/$(bin_target)/debug/%, $(bin_names))

.PHONY: filesystem

filesystem: xargo-bin copy-bins

xargo-bin:
	@cd filesystem; xargo build --target=$(bin_target) --bins

clean-filesystem:
	@xargo clean
	@rm -r filesystem/target/

copy-bins:
	@mkdir -p build/isofiles/bin
	@cp $(bin_files) build/isofiles/bin
	@cp -r filesystem/precompiled/ build/isofiles/bin