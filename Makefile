cargo_prepare:
	@cargo -V &> /dev/null

bootimage_prepare:
	cargo install bootimage

qemu_prepare:
	@qemu-system-x86_64 --version &> /dev/null

bochs:
	@bochs -h &> /dev/null

prepare: cargo_prepare bootimage_prepare qemu_prepare;

build: prepare;
	@cargo bootimage --target target.json

build_release: prepare;
	@cargo bootimage --release --target target.json

clean: cargo_prepare;
	@cargo clean

bin2img:
	@dd if=./target/target/debug/bootimage-nonamekernel.bin of=./target/target/debug/bochs.img conv=notrunc

start_qemu: build ;
	@qemu-system-x86_64 -s -drive format=raw,file=./target/target/debug/bootimage-nonamekernel.bin

start_release_qemu: build_release;
	@qemu-system-x86_64 -s -drive format=raw,file=./target/target/release/bootimage-nonamekernel.bin

start: build bin2img;
	@rm ./target/target/debug/bochs.img.lock
	@bochs -q

start_release: build_release bin2img;
	@bochs -q
