#!/usr/bin/env bash

xtensa-esp32s3-elf-gcc ./csrc/main.c \
    -Wl,-r                           \
    -Wl,-e,main                      \
    -nostartfiles                    \
    -nodefaultlibs                   \
    -nostdlib                        \
    -o ./csrc/c.elf
