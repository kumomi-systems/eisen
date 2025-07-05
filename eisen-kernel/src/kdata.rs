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

use eisen_lib::boot::sysinfo::SysInfo;

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".kdata")]
#[used]
static _kargs: [u8; 0x400] = [0; 0x400];

pub fn kargs() -> &'static str {
  core::str::from_utf8(&_kargs).unwrap()
}

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".kdata")]
#[used]
static _ksysinfo: [u8; size_of::<SysInfo>()] = [0; size_of::<SysInfo>()];

pub fn ksysinfo() -> &'static SysInfo {
  unsafe {
    (&_ksysinfo as *const [u8; size_of::<SysInfo>()] as *const SysInfo).as_ref().unwrap()
  }
}