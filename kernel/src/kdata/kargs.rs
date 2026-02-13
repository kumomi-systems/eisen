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

pub const KARGS_SIZE: usize = 2;

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".kdata")]
#[used]
static _kargs: [u8; KARGS_SIZE] = [0; KARGS_SIZE];

#[cfg(feature = "kernel-only")]
pub fn kargs() -> &'static str {
  unsafe { core::str::from_utf8_unchecked(&_kargs) }
}