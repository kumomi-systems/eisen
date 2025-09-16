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

use core::u32;

use eisen_lib::boot::bootinfo::BootInfo;
use eisen_lib::boot::UEFI_KERNEL_MEM_TYPE;
use uefi::boot::{MemoryAttribute, MemoryDescriptor, MemoryType, PAGE_SIZE};
use uefi::mem::memory_map::{MemoryMapMut, MemoryMapOwned};
use uefi::runtime::set_virtual_address_map;
use uefi::table::system_table_raw;
use uefi::{Error, Status};
use uefi_raw::table::system::SystemTable;

pub const TEMP_MAP_MEMORY_TYPE: u32 = 0xEBBA003C;

pub fn set_virt_map(bootinfo: &BootInfo, mem_map: &mut MemoryMapOwned) -> Status {
  let mut mem_map_idx = 0;
  let mut mem_map_entry: Option<&mut MemoryDescriptor>;

  let vma_max         = u64::MAX as u64;
  let mut vma_offset  = bootinfo.kernel_vma;
  let mut vmem_free   = vma_max - vma_offset;

  loop {
    mem_map_entry = mem_map.get_mut(mem_map_idx);
    mem_map_idx += 1;

    if mem_map_entry.is_none() {
      break;
    }
    let entry = mem_map_entry.unwrap();

    match entry.ty {
      // MemoryType(UEFI_KERNEL_MEM_TYPE) => {
      //   entry.virt_start  = bootinfo.kernel_vma;
      // }
      MemoryType::LOADER_CODE |
      MemoryType::LOADER_DATA |
      MemoryType::BOOT_SERVICES_CODE |
      MemoryType::BOOT_SERVICES_DATA |
      wakatiwai_udive::BOOT_DRIVER_IO_MEMTYPE |
      wakatiwai_udive::FSYS_DRIVER_IO_MEMTYPE => {
        entry.ty          = MemoryType::CONVENTIONAL;
        entry.virt_start  = entry.phys_start;
      }
      _ => {
        entry.virt_start  = entry.phys_start;
      }
    }
  }

  Status::SUCCESS
}

pub unsafe fn load_virt_map(descriptors: &mut [MemoryDescriptor]) -> Result<(), Error> {
  let mut sys_table_ptr = system_table_raw().unwrap().as_ptr();
  let _ = ((*(*sys_table_ptr).runtime_services).convert_pointer)(
    0 as usize,
    (&mut sys_table_ptr)
      as *mut *mut SystemTable
      as *mut *mut core::ffi::c_void
      as *mut *const core::ffi::c_void
  );
  set_virtual_address_map(
    descriptors,
    sys_table_ptr as *const SystemTable
  )
}