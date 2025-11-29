#!/bin/bash

qemu-system-aarch64 \
  -machine virt \
  -cpu cortex-a72 \
  -m 1024 \
  -nographic \
  -kernel target/aarch64-unknown-none/debug/mainframe-kernel
