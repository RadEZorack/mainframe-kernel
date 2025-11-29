#!/bin/bash

KERNEL=target/aarch64-unknown-none/debug/mainframe-kernel

# If hyphen version doesn't exist, try underscore version
if [ ! -f "$KERNEL" ]; then
    KERNEL=target/aarch64-unknown-none/debug/mainframe_kernel
fi

# If still missing, list directory and bail out
if [ ! -f "$KERNEL" ]; then
    echo "❌ Kernel binary not found!"
    echo "Here is what exists:"
    ls target/aarch64-unknown-none/debug
    exit 1
fi

echo "🟢 Booting kernel: $KERNEL"

QEMU=$(which qemu-system-aarch64)

/usr/bin/env "$QEMU" \
  -machine virt \
  -cpu cortex-a72 \
  -m 1024 \
  -kernel "$KERNEL" \
  -nographic \
  -serial mon:stdio \
  -serial null
