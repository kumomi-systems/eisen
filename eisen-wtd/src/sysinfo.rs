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


use eisen_lib::boot::sysinfo::*;
use uefi::{proto::console::gop::GraphicsOutput, Identify};
use uefi_raw::{protocol::console::{GraphicsOutputModeInformation, GraphicsOutputProtocol}, table::system::SystemTable};

pub unsafe fn load_system_information() -> SysInfo {
  let systable = unsafe { uefi::table::system_table_raw().unwrap().as_ref() };

  SysInfo {
    acpi_addr:        (*systable.configuration_table).vendor_table,
    acpi_entries:     systable.number_of_configuration_table_entries,

    graphicsinfo:     load_graphics_information(systable),
    meminfo:          load_memory_information()
  }
}

unsafe fn load_graphics_information(systable: &SystemTable) -> SysGraphicsInfo {
  let mut gop: *mut GraphicsOutputProtocol = core::ptr::null_mut();
  #[allow(const_item_mutation)]
  let _ = ((*systable.boot_services).locate_protocol)(
    &mut GraphicsOutput::GUID as *mut _,
    core::ptr::null_mut(),
    &mut gop as *mut _ as *mut *mut core::ffi::c_void
  );

  let mut best_ratio: f32                 = 0.0;
  let mut best_resolution: u64            = 0;
  let mut best_mode: u32                  = (*(*gop).mode).mode;
  let mut info_size: usize                = size_of::<GraphicsOutputModeInformation>();
  let mut info: *const GraphicsOutputModeInformation = 0 as *mut _;
  for x in 0..(*(*gop).mode).max_mode {
    if ((*gop).query_mode)(
      gop,
      x,
      &mut info_size,
      &mut info
    ).is_success() {
      let hres = (*info).horizontal_resolution;
      let vres = (*info).vertical_resolution;
      let mode_resolution = hres as u64 * vres as u64;
      let mode_ratio = hres as f32 / vres as f32;

      if
        mode_resolution > best_resolution &&
        mode_resolution <= 1200*800       &&
        mode_ratio >= best_ratio          &&
        mode_ratio <= 16.0/9.0
      {
        best_mode = x;
        best_resolution = mode_resolution;
        best_ratio = mode_ratio;
      }
    }
  }

  let _ = ((*gop).set_mode)(gop, best_mode);

  SysGraphicsInfo {
    framebuffer_base: (*(*gop).mode).frame_buffer_base as *mut u8,
    framebuffer_size: (*(*gop).mode).frame_buffer_size,
    resolution:       ((*(*(*gop).mode).info).horizontal_resolution, (*(*(*gop).mode).info).vertical_resolution),
    pixels_per_line:  (*(*(*gop).mode).info).pixels_per_scan_line,
    pixel_info:       (*(*(*gop).mode).info).pixel_format.0 as u8
  }
}

unsafe fn load_memory_information() -> SysMemInfo {
  SysMemInfo {

  }
}