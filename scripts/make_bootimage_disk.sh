KERNEL_HDD="linux_hdd.hdd"
PATH_ELF="build/target/debug/bootimage-nonamekernel.bin"

dd if=${PATH_ELF} of=${KERNEL_HDD}