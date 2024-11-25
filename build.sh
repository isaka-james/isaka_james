#!/bin/bash

set -e

# Build the kernel
cargo build --release --target x86_64-unknown-none

# Copy kernel directly (no objcopy needed)
cp target/x86_64-unknown-none/release/isaka_james iso/boot/kernel.bin

# Create ISO
grub-mkrescue -o isaka_james.iso iso/

# Run in QEMU
qemu-system-x86_64 -cdrom isaka_james.iso -boot d -serial stdio

