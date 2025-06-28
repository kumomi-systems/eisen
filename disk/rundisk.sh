#!/bin/bash

RT=$(realpath $(dirname $0)/..)
cd $RT

qemu-system-x86_64					\
	-L OVMF/                	\
	-pflash OVMF/OVMF.4m.fd		\
	-net none               	\
	-usb Eisen.img           	\
	-vga std                	\
	-enable-kvm								\
	-cpu host									\
	-m 4G
