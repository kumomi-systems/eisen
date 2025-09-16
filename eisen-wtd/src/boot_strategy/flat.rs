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

use eisen_lib::boot::{bootinfo::BootInfo, UEFI_KERNEL_MEM_TYPE};
use uefi::{boot::{allocate_pages, allocate_pool, exit_boot_services, MemoryDescriptor, MemoryType, PAGE_SIZE}, mem::memory_map::MemoryMap, Status};
use wakatiwai_udive::boot::BootDriverArgs;

use crate::{paging::*, sysinfo::{load_system_information, reload_memory_information}};

pub fn boot(args: &BootDriverArgs, bootinfo: &BootInfo) -> Option<Status> {
  unsafe {
    let kernel_phys = allocate_pages(
      uefi::boot::AllocateType::AnyPages,
      MemoryType(UEFI_KERNEL_MEM_TYPE),
      (bootinfo.kernel_size as usize + (PAGE_SIZE - 1)) / PAGE_SIZE
    ).unwrap();

    core::ptr::copy(
      args.img.as_ptr().add(bootinfo.header_size as usize),
      kernel_phys.as_ptr(),
      bootinfo.kernel_size as usize
    );

    let mut mm = uefi::boot::memory_map(MemoryType::LOADER_CODE).unwrap();
    set_virt_map(bootinfo, &mut mm);

    for entry in mm.entries() {
      uefi::println!(
        "phys: {:#018X}, virt: {:#018X} ({:#010X} pages) - {:?}, {:?}",
        entry.phys_start,
        entry.virt_start,
        entry.page_count,
        entry.ty,
        entry.att
      );
    }

    let mut sysinfo = load_system_information();

    uefi::println!("Exiting boot services...");
    let mem_map = exit_boot_services(MemoryType::custom(TEMP_MAP_MEMORY_TYPE));
    load_virt_map(
      core::slice::from_raw_parts_mut(
        mem_map.buffer().as_ptr() as *mut MemoryDescriptor,
        mem_map.len()
      )
    );
    reload_memory_information(&mut sysinfo.meminfo, &mem_map);

    core::arch::asm!(
      r#"
        call {}
      "#,
      in(reg) kernel_phys.as_ptr() as u64
    );
  }
  None
}