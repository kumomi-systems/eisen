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

use eisen_lib::boot::bootinfo::BootInfo;
use elf::{abi::PT_LOAD, endian::NativeEndian, ElfBytes};
use uefi::{boot::{allocate_pages, MemoryDescriptor, PAGE_SIZE}, println, Status};
use wakatiwai_udive::boot::BootDriverArgs;

pub fn boot(args: &BootDriverArgs, bootinfo: &BootInfo) -> Option<Status> {
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
    // let sysinfo = crate::sysinfo::load_system_information();
    // crate::paging::enable_paging(bootinfo);
    // // *(bootinfo.ksysinfo.as_ptr()) = sysinfo;
    // core::arch::asm!(
    //   r#"
    //     call {}
    //   "#,
    //   in(reg) kernel_elf.ehdr.e_entry,
    // );
    todo!()
  }

  unreachable!()
}