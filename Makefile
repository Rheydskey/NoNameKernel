cargo_prepare:
	@cargo -V &> /dev/null

bootimage_prepare:
	cargo install bootimage

qemu_prepare:
	@qemu-system-x86_64 --version &> /dev/null

prepare: cargo_prepare bootimage_prepare qemu_prepare;

build: prepare;
	@cargo bootimage --target target.json

build_release: prepare;
	@cargo bootimage --release --target target.json

clean: cargo_prepare;
	@cargo clean

start: build;
	@qemu-system-x86_64 -drive format=raw,file=./target/target/debug/bootimage-nonamekernel.bin

start_release: build_release;
	@qemu-system-x86_64 -drive format=raw,file=./target/target/release/bootimage-nonamekernel.bin