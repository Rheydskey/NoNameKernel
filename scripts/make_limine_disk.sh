KERNEL_HDD="linux_hdd.hdd"
PATH_ELF=$1

echo $PATH_ELF

rm -rvf iso_root
mkdir -p iso_root

# Copy the relevant files over.
cp -v $PATH_ELF bin/limine.cfg limine/limine.sys \
limine/limine-cd.bin limine/limine-cd-efi.bin iso_root/

# Create the bootable ISO.
xorriso -as mkisofs -b limine-cd.bin \
-no-emul-boot -boot-load-size 4 -boot-info-table \
--efi-boot limine-cd-efi.bin \
-efi-boot-part --efi-boot-image --protective-msdos-label \
iso_root -o ${KERNEL_HDD}

# Install Limine stage 1 and 2 for legacy BIOS boot.
./limine/limine-deploy ${KERNEL_HDD}