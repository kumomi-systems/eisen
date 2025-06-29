#!/bin/bash

KERNEL=$1
BIHEADER=$KERNEL.biheader
KERNELTMP=$KERNEL.tmp

echo "Signing kernel binary $KERNEL..."

BOOTINFO_SIZE=512
BOOTINFO_OFFSET=512

to_little_endian() {
  v=$1
  echo "${v:6:2}${v:4:2}${v:2:2}${v:0:2}"
}

# Extract bootinfo
BOOTINFO=$(xxd -s $BOOTINFO_OFFSET -l $BOOTINFO_SIZE -g0 -ps -c0 $KERNEL)
BOOTINFO_SPLIT=$(echo $BOOTINFO | fold -w8 | tr ' ' '\n')
update_bootinfo() {
  BOOTINFO=$(echo $BOOTINFO_SPLIT | tr -d ' ')
  echo $BOOTINFO | xxd -r -p > $BIHEADER
  BOOTINFO_SPLIT=$(echo $BOOTINFO | fold -w8 | tr ' ' '\n')
}
update_bootinfo

# Insert product UUID
UUID=$(uuidgen)
UUID_FLAT=$(echo $UUID | tr -d '-' | fold -w8)
UUID_IDX=4
for UUID_PART in ${UUID_FLAT}; do
  BOOTINFO_SPLIT=$(echo $BOOTINFO_SPLIT | tr ' ' '\n' | sed "$(($UUID_IDX+1))""s/.*/$UUID_PART/")
  UUID_IDX=$((UUID_IDX+1))
done
update_bootinfo
echo "Inserted product UUID: $UUID"

# Generate checksum
CHECKSUM=$(rhash --crc32 $BIHEADER | tail -n1 | awk '{print $2}')
BOOTINFO_SPLIT=$(echo $BOOTINFO_SPLIT | tr ' ' '\n' | sed "121s/.*/$(to_little_endian $CHECKSUM)/")
update_bootinfo
echo "Inserted CRC32 checksum: $CHECKSUM"

# Rebuild kernel
head -c$BOOTINFO_OFFSET $KERNEL > $KERNELTMP
cat $BIHEADER >> $KERNELTMP
tail -c+$(($BOOTINFO_OFFSET + $BOOTINFO_SIZE)) $KERNEL >> $KERNELTMP
mv $KERNELTMP $KERNEL
rm $BIHEADER

echo "Kernel binary $KERNEL signed successfully!"