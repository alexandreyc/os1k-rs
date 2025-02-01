#!/usr/bin/env bash

set -xue

QEMU="qemu-system-riscv32"
KERNEL="target/riscv32i-unknown-none-elf/debug/os1k"

# Build the kernel
cargo build

# Start QEMU
$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot \
    -kernel ${KERNEL}
