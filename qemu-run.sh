#!/bin/bash
set -euo pipefail

TARGET_TRIPLE="aarch64-unknown-none"
BUILD_PROFILE="debug"
BUILD_DIR="target/${TARGET_TRIPLE}/${BUILD_PROFILE}"
KERNEL_ELF="${BUILD_DIR}/mainframe-kernel"
KERNEL_BIN="${KERNEL_ELF}.bin"

find_objcopy() {
    if command -v rust-objcopy >/dev/null 2>&1; then
        echo "rust-objcopy"
        return
    fi

    if command -v llvm-objcopy >/dev/null 2>&1; then
        echo "llvm-objcopy"
        return
    fi

    if OBJ=$(rustup which --toolchain nightly rust-objcopy 2>/dev/null); then
        echo "$OBJ"
        return
    fi

    if OBJ=$(rustup which --toolchain nightly llvm-objcopy 2>/dev/null); then
        echo "$OBJ"
        return
    fi

    echo ""
}

OBJCOPY_BIN=$(find_objcopy)

if [ -z "$OBJCOPY_BIN" ]; then
    cat <<'EOF'
❌ Could not find rust-objcopy or llvm-objcopy.
➡️  Install the LLVM tools component:
    rustup component add llvm-tools-preview
EOF
    exit 1
fi

if [ ! -f "$KERNEL_ELF" ]; then
    echo "❌ Kernel ELF not found at ${KERNEL_ELF}"
    echo "➡️  Run: cargo build --target ${TARGET_TRIPLE}"
    exit 1
fi

if [ ! -f "$KERNEL_BIN" ] || [ "$KERNEL_ELF" -nt "$KERNEL_BIN" ]; then
    echo "📦 Generating raw image ${KERNEL_BIN}"
    "$OBJCOPY_BIN" \
        --binary-architecture=aarch64 \
        --strip-all \
        -O binary \
        "$KERNEL_ELF" "$KERNEL_BIN"
fi

echo "🟢 Booting kernel image: $KERNEL_BIN"

qemu-system-aarch64 \
    -machine virt \
    -cpu cortex-a72 \
    -m 512 \
    -kernel "$KERNEL_BIN" \
    -bios none \
    -serial mon:stdio \
    -nographic \
    -no-reboot
