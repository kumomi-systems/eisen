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

use crate::arch::x86_64::ports::{Port, outb};

use core::fmt::{Write, Result};

#[cfg(debug_assertions)]
pub struct DebugWriter;

#[cfg(debug_assertions)]
impl Write for DebugWriter {
  #[cfg(target_arch = "x86_64")]
  fn write_char(&mut self, c: char) -> Result {
    unsafe { outb(&Port::DebugOut, c as u8); }

    Result::Ok(())
  }

  fn write_str(&mut self, s: &str) -> Result {
    for c in s.chars() {
      self.write_char(c).unwrap();
    }

    Result::Ok(())
  }
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug {
  () => {};
  ($($arg:tt)*) => {{
    use core::fmt::Write;
    write!(crate::debug::DebugWriter, $($arg)*).unwrap();
  }}
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debugln {
  () => {{
    use core::fmt::Write;
    write!(crate::debug::DebugWriter, '\n').unwrap();
  }};
  ($($arg:tt)*) => {{
    use core::fmt::Write;
    write!(crate::debug::DebugWriter, "[DEBUG] ").unwrap();
    $crate::debug!($($arg)*);
    write!(crate::debug::DebugWriter, "\n").unwrap();
  }}
}