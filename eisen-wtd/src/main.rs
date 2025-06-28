#![no_main]
#![no_std]

use eisen_lib::bootinfo::{self, BootInfo};
use uefi::println;

wakatiwai_udive::boot_prelude!();

fn main(args: &BootDriverArgs) -> Option<Status> {
  let bootbuf: [u8; 0x200] = (&args.img[0x200..0x400]).try_into().unwrap();
  let bootinfo = bootinfo::BootInfo::parse(bootbuf);
  
  None
}