#!/bin/bash

STUB=$1
KERNEL_ELF=$2

# Extract binary stub
objcopy                 \
  -j .mbrtrap           \
  -j .bootinfo          \
  -I elf64-x86-64       \
  -O binary             \
  $KERNEL_ELF $STUB

# Remove unwanted sections
objcopy                 \
  -R .mbrtrap           \
  -R .bootinfo          \
  -R .late_discard      \
  -F elf64-x86-64       \
  $KERNEL_ELF