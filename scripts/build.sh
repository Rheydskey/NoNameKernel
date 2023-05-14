#!/usr/bin/bash
KERNEL=$1
./scripts/make_limine_disk.sh $KERNEL
qemu-system-x86_64 -serial stdio -drive format=raw,file=linux_hdd.hdd
rm -rvf iso_root