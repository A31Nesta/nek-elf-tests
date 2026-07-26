#!/usr/bin/env bash

TARGET_DIR="target/xtensa-esp32s3-none-elf/release"
TARGET_CRATE="$1"

cargo clean -p nek-sys
cargo build --release -p $TARGET_CRATE

# Remove evil and intimidating LLVM stuff
xtensa-esp32s3-elf-objcopy \
    --remove-section=.llvmbc \
    --remove-section=.llvmcmd \
    $TARGET_DIR/$TARGET_CRATE \
    $TARGET_DIR/$TARGET_CRATE
