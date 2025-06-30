## Eisen Operating System
## Copyright (C) 2025  Kumomi Systems
## 
## This program is free software: you can redistribute it and/or modify
## it under the terms of the GNU General Public License as published by
## the Free Software Foundation, either version 3 of the License, or
## (at your option) any later version.
## 
## This program is distributed in the hope that it will be useful,
## but WITHOUT ANY WARRANTY; without even the implied warranty of
## MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
## GNU General Public License for more details.
## 
## You should have received a copy of the GNU General Public License
## along with this program. If not, see <https://www.gnu.org/licenses/>.

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