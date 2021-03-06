KERNEL_HDD="linux_hdd.hdd"
PATH_ELF="build/target/debug/nonamekernel"

echo "${KERNEL_HDD}"
rm -f ${KERNEL_HDD}
dd if=/dev/zero bs=1M count=0 seek=64 of=${KERNEL_HDD}
parted -s ${KERNEL_HDD} mklabel gpt
parted -s ${KERNEL_HDD} mkpart primary 2048s 100%
echfs-utils -g -p0 ${KERNEL_HDD} quick-format 512
echfs-utils -g -p0 ${KERNEL_HDD} import ${PATH_ELF} stivale.elf
echfs-utils -g -p0 ${KERNEL_HDD} import submodules/limine.cfg limine.cfg
echfs-utils -g -p0 ${KERNEL_HDD} import submodules/limine/limine.sys limine.sys
./submodules/limine/limine-install-linux-x86_64 ${KERNEL_HDD}