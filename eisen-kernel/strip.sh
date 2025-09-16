#!/bin/bash

STUB=$1
KERNEL=$2
KERNEL_ELF=$3

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
  -I elf64-x86-64       \
  -O binary             \
  $KERNEL_ELF $KERNEL