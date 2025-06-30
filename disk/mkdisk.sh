#!/bin/bash

if [ "$(id -u)" -ne 0 ]; then
	echo "This script must be run as root." 1>&2
	exit 1
fi

LOGNAME=$(logname)

export RT=$(realpath $(dirname $0)/..)
export BIN="$RT/bin"
export DSK="$RT/disk"
export ISO="$RT/iso"
export IMG="Eisen.img"

cd $RT

# Build all binaries
sudo -u $LOGNAME make &
BUILD_PID=$!

# Produce a disk image in preparation for compiled binaries
sudo -u $LOGNAME dd if=/dev/zero of=$IMG bs=1G count=4
gdisk $IMG < $DSK/gdiskcmds > /dev/null
partprobe

# Setup a loop device
LODEV=$(losetup -f)
EFI_PART="$LODEV"p1
EISEN_PART="$LODEV"p2
MNTPNT="$RT/mnt"

losetup -P $LODEV $IMG
mkdir -p $MNTPNT

# Create file systems
mkfs.fat -F 32 -n "SYSEFI"  $EFI_PART
mkfs.fat -F 32 -n "EISEN"   $EISEN_PART

# Wait for the build process to finish before populating the disk
wait $BUILD_PID

# Install bootloader
mount $EFI_PART $MNTPNT
wakatiwai-install $MNTPNT $DSK/wakatiwai/driverlist
mkdir $MNTPNT/EFI/BOOT
mv $MNTPNT/EFI/wakatiwai/wakatiwai.efi $MNTPNT/EFI/BOOT/BOOTX64.EFI
cp $DSK/wakatiwai/wtconfig.json $MNTPNT/EFI/wakatiwai/wtconfig.json
tree $MNTPNT
umount $EFI_PART

# Install OS
mount $EISEN_PART $MNTPNT
cp $BIN/kernel.bin $MNTPNT/kernel
tree $MNTPNT
umount $EISEN_PART

losetup -D