#!/bin/bash

if [ "$(id -u)" -ne 0 ]; then
	echo "This script must be run as root." 1>&2
	exit 1
fi

RT=$(realpath $(dirname $0)/..)

PACKAGES=(
	rustup				# Rust
	binutils			# as, ld

	gdisk					# Disk partitioning
	rhash					# Header signing

	qemu-desktop	# Virtualisation
	ovmf					# UEFI firmware

	tinyxxd				# Provides xxd
)

pacman -Syy
pacman -S $PACKAGES --needed --noconfirm

rustup default nightly
rustup target install x86_64-unknown-none
rustup target add x86_64-unknown-none
rustup component add llvm-tools
rustup update

cp -r /usr/share/OVMF/x64 $RT/OVMF
chown -R --reference=$RT $RT/OVMF