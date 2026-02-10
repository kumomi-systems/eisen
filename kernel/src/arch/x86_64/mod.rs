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

pub mod gdt;
pub mod int;
pub mod mem;
pub mod ports;

use core::ffi::c_void;

use x86_64::instructions::interrupts;

use crate::debugln;

unsafe extern "C" {
  static STACK_TOP: *const c_void; 
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".kentry.entry")]
unsafe extern "C" fn _kentry() -> ! {
  debugln!("Kernel entry");
  
  interrupts::disable();
  debugln!("Disabled interrupts");
  
  core::arch::asm!(
    "mov {}, rsp",
    in(reg) STACK_TOP
  );
  debugln!("Initialised stack");

  gdt::load_gdt();
  int::load_interrupts();
  interrupts::enable();
  debugln!("Enabled interrupts");

  crate::_kmain();
}