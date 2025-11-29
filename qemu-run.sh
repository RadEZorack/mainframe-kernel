#!/bin/bash

set -euo pipefail

KERNEL=${1:-target/aarch64-unknown-none/debug/mainframe-kernel}

if [[ ! -f "$KERNEL" ]]; then
    echo "Kernel binary not found: $KERNEL" >&2
    exit 1
fi

echo "🟢 Booting: $KERNEL"

# qemu-system-aarch64 \
#     -machine virt,secure=off,virtualization=on,gic-version=3 \
#     -cpu cortex-a72 \
#     -m 512 \
#     -serial mon:stdio \
#     -device ramfb \
#     -display cocoa \
#     -kernel "$KERNEL" \
#     -no-reboot \
#     ${QEMU_EXTRA_ARGS:-}


qemu-system-aarch64 \
    -machine virt \
    -cpu cortex-a72 \
    -m 512 \
    -display cocoa \
    -device ramfb \
    -kernel target/aarch64-unknown-none/debug/mainframe-kernel \
    -serial mon:stdio
