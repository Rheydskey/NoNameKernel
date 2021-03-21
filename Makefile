KERNEL_HDD = disk.hdd

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
	rm -f $(KERNEL_HDD)

bin2img:
	@dd if=./target/target/debug/bootimage-nonamekernel.bin of=./target/target/debug/bochs.img conv=notrunc

start_qemu: build ;
	@qemu-system-x86_64 -s -drive format=raw,file=./target/target/debug/bootimage-nonamekernel.bin

start_release_qemu: build_release;
	@qemu-system-x86_64 -s -drive format=raw,file=./target/target/release/bootimage-nonamekernel.bin

start: build bin2img;
	@bochs -q

start_release: build_release bin2img;
	@bochs -q


.PHONY: clean all run

all: $(KERNEL_HDD)

run: $(KERNEL_HDD)
	qemu-system-x86_64 -m 2G -hda $(KERNEL_HDD)

limine:
	make -C submodules/limine

echfs:
	git clone https://github.com/echfs submodules/echfs
	make -C submodules/echfs
	sudo make -C submodules/echfs install

$(KERNEL_HDD): limine
	rm -f $(KERNEL_HDD)
	dd if=/dev/zero bs=1M count=0 seek=64 of=$(KERNEL_HDD)
	parted -s $(KERNEL_HDD) mklabel gpt
	parted -s $(KERNEL_HDD) mkpart primary 2048s 100%
	echfs-utils -g -p0 $(KERNEL_HDD) quick-format 512
	echfs-utils -g -p0 $(KERNEL_HDD) import ./submodules/limine.cfg limine.cfg
	echfs-utils -g -p0 $(KERNEL_HDD) import ./submodules/limine/limine.sys limine.sys
	echfs-utils -g -p0 $(KERNEL_HDD) import ./target/target/debug/nonamekernel boot/test.elf
	./submodules/limine/limine-install $(KERNEL_HDD)
