KERNEL_HDD="linux_hdd.hdd"
PATH_ELF="build/target/debug/nonamekernel"
PATH_BUILD="build/grub"

echo "${KERNEL_HDD}"
rm -f ${KERNEL_HDD}

mkdir -p ${PATH_BUILD}/boot
echo "set timeout=15
set default=0

menuentry \"NoNameKernel\" {
    multiboot /boot/nonamekernel
}
" >> ${PATH_BUILD}/grub.cfg

cp ${PATH_ELF} ${PATH_BUILD}/boot

grub-mkrescue -o ${KERNEL_HDD} ${PATH_BUILD}
