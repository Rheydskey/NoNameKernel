#!/usr/bin/bash
cargo build
./scripts/make_limine_disk.sh
qemu-system-x86_64 -serial stdio -drive format=raw,file=linux_hdd.hdd