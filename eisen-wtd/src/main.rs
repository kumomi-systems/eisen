// Eisen Operating System
// Copyright (C) 2025  Kumomi Systems
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![no_main]
#![no_std]
#![feature(breakpoint)]

extern crate alloc;

mod elf;
mod sysinfo;

use eisen_lib::boot::bootinfo::{BootInfo, KernelType};
use uefi::{println, proto::console::text::*};

wakatiwai_udive::boot_prelude!();

fn main(args: &BootDriverArgs) -> Option<Status> {
  uefi::system::with_stdout(|proto| {
    Output::clear(proto).unwrap();
    Output::set_color(
      proto,
      Color::White,
      Color::Black
    ).unwrap();
  });
  println!("Eisen Bootloader");

  let bootinfo: BootInfo;
  match BootInfo::parse((args.img[0x200..0x400]).try_into().unwrap()) {
    Ok(ok) => {
      bootinfo = ok;
      println!("Validated kernel header");
    }
    Err(err) => {
      println!("Failed to validate kernel header: {:?}", err);
      return Some(Status::ABORTED);
    }
  }
  
  let checksum = bootinfo.checksum;
  println!("Header information:");
  println!("  UUID:         {:#X}", uuid::Uuid::from_bytes(bootinfo.install_uuid));
  println!("  Version:      {}", bootinfo.version_info());
  println!("  Built:        {}", bootinfo.date());
  println!("  Checksum:     {:X}", checksum);
  println!("  Stub End:     {:#X}", bootinfo.stub_end.abs_diff(0));

  match bootinfo.kernel_type {
    KernelType::Elf => {
      elf::load_elf_kernel(args, &bootinfo)
    }
    _ => {
      todo!()
    }
  }
}