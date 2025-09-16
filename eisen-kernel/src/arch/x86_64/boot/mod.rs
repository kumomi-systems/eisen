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

use x86_64::instructions::interrupts;

use crate::debugln;

unsafe extern "C" {
  static STACK_TOP: *const c_void; 
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".kentry")]
unsafe extern "C" fn _kentry() -> ! {
  loop {}
  debugln!("Kernel entry (x86_64)");
  
  interrupts::disable();
  debugln!("Disabled interrupts");
  
  core::arch::asm!(
    "mov {}, rsp",
    in(reg) STACK_TOP
  );
  debugln!("Initialised stack");

  super::gdt::load_gdt();
  super::int::load_interrupts();
  interrupts::enable();
  debugln!("Enabled interrupts");

  crate::_kmain();
}