#!/usr/bin/bash
cargo build --release
./scripts/make_limine_disk_release.sh
qemu-system-x86_64 -serial stdio -drive format=raw,file=linux_hdd.hdd