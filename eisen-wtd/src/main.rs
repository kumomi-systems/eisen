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

extern crate alloc;

use eisen_lib::boot::{bootinfo::BootInfo, sysinfo::SysInfo};
use uefi::{boot::allocate_pages, println, proto::console::text::*};

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
      println!("Validated kernel header.");
    }
    Err(err) => {
      println!("Failed to validate kernel header: {:?}", err);
      return Some(Status::ABORTED);
    }
  }
  
  let checksum = bootinfo.checksum;
  println!("Header information:");
  println!("  UUID:     {:#X}", uuid::Uuid::from_bytes(bootinfo.install_uuid));
  println!("  Version:  {}", bootinfo.version_info());
  println!("  Built:    {}", bootinfo.date());
  println!("  Checksum: {:X}", checksum);

  println!("Loading kernel...");
  println!("  Kernel load address:  {:#010X}", bootinfo.kloadaddr.abs_diff(0));
  println!("  Kernel size:          {} ({} pages)", bootinfo.kernel_size_pretty(), (bootinfo.kernel_size as usize + uefi::boot::PAGE_SIZE - 1) / uefi::boot::PAGE_SIZE);
  let kernel_phys_addr: *mut u8;
  match allocate_pages(
    uefi::boot::AllocateType::Address(bootinfo.kloadaddr),
    uefi::boot::MemoryType::LOADER_DATA,
    (bootinfo.kernel_size as usize + uefi::boot::PAGE_SIZE - 1) / uefi::boot::PAGE_SIZE
  ) {
    Ok(ok) => {
      kernel_phys_addr = ok.as_ptr();
    }
    Err(err) => {
      println!("Unable to load kernel to requested address: {:?}", err.status());
      return Some(Status::ABORTED);
    }
  }
  unsafe {
    let kernel = args.img.clone();
    core::slice::from_raw_parts_mut(kernel_phys_addr, bootinfo.kernel_size as usize).copy_from_slice(&kernel[..]);
  }

  println!("Setting system information...");
  let sysinfo: SysInfo;
  unsafe {
    let sys_table_raw = uefi::table::system_table_raw().unwrap().as_mut();
    sysinfo = SysInfo {
      acpi_addr:      (*sys_table_raw.configuration_table).vendor_table,
      acpi_entries:   sys_table_raw.number_of_configuration_table_entries
    };
    *(bootinfo.ksysinfo.as_ptr()) = sysinfo;
  }

  println!("Jumping to kernel...");
  unsafe {
    let kentry = bootinfo.kentry;
    println!("Kernel location: {:#010X}", kentry.as_ptr().addr());
    println!(
      "First four bytes: {:#04X} {:#04X} {:#04X} {:#04X}",
      *((kentry.as_ptr() as *mut u8).add(0)),
      *((kentry.as_ptr() as *mut u8).add(1)),
      *((kentry.as_ptr() as *mut u8).add(2)),
      *((kentry.as_ptr() as *mut u8).add(3))
    );
    
    kentry.as_ref()();
  }
}