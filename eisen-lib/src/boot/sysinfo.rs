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

use core::ffi::c_void;

#[derive(Debug)]
pub struct SysInfo {
  pub acpi_addr:    *const c_void,
  pub acpi_entries: usize,

  pub graphicsinfo: SysGraphicsInfo,
  pub meminfo:      SysMemInfo
}

impl SysInfo {
  pub const fn new() -> Self {
    Self {
      acpi_addr:    0 as *const c_void,
      acpi_entries: 0,

      graphicsinfo: SysGraphicsInfo::new(),
      meminfo:      SysMemInfo::new()
    }
  }
}

#[derive(Debug)]
pub struct SysGraphicsInfo {
  pub framebuffer_base: *mut u8,
  pub framebuffer_size: usize,
  pub resolution:       (u32, u32),
  pub pixels_per_line:  u32,
  pub pixel_info:       u8
}

impl SysGraphicsInfo {
  pub const fn new() -> Self {
    Self {
      framebuffer_base: 0 as *mut u8,
      framebuffer_size: 0,
      resolution:       (0, 0),
      pixels_per_line:  0,
      pixel_info:       0
    }
  }
}

#[derive(Debug)]
pub struct SysMemInfo {

}

impl SysMemInfo {
  pub const fn new() -> Self {
    Self {
      
    }
  }
}