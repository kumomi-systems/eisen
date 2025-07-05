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

mod sysinfo;

use eisen_lib::boot::bootinfo::BootInfo;
use elf::{abi::PT_LOAD, endian::NativeEndian, ElfBytes};
use uefi::{boot::{allocate_pages, PAGE_SIZE}, println, proto::console::text::*};

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

  let kernel_elf: ElfBytes<NativeEndian>;
  match ElfBytes::<NativeEndian>::minimal_parse(
    &args.img[bootinfo.stub_end as usize..]
  ) {
    Ok(ok) => {
      println!("Successfully parsed kernel ELF");
      kernel_elf = ok;
    }
    Err(err) => {
      println!("Failed to parse kernel ELF: {:?}", err);
      return Some(Status::ABORTED);
    }
  }

  for segment in kernel_elf.segments().unwrap() {
    println!("offset: {:#010X}, lma: {:#010X}, vma: {:#010X}", segment.p_offset, segment.p_paddr, segment.p_vaddr);
    if segment.p_type == PT_LOAD {
      let segment_pages = (segment.p_memsz as usize + PAGE_SIZE - 1) / PAGE_SIZE;
      let segment_load_addr = allocate_pages(
        uefi::boot::AllocateType::Address(segment.p_vaddr),
        uefi::boot::MemoryType::LOADER_DATA,
        segment_pages
      ).unwrap();

      unsafe {
        core::ptr::copy_nonoverlapping(
          args.img.as_ptr()
            .add(bootinfo.stub_end as usize)
            .add(segment.p_offset as usize),
          segment_load_addr.as_ptr(),
          segment.p_filesz as usize
        );
      }
    }
  }

  unsafe {
    *(bootinfo.ksysinfo.as_ptr()) = crate::sysinfo::load_system_information();

    println!("Jumping to kernel entry point @ {:#010X}...", kernel_elf.ehdr.e_entry);
    let _ = uefi::boot::exit_boot_services(uefi::boot::MemoryType::LOADER_DATA);
    core::arch::asm!(
      r#"
        call {}
      "#,
      in(reg) kernel_elf.ehdr.e_entry,
    );
  }

  unreachable!()
}