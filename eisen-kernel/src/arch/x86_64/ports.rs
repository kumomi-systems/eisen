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


// Good list here: https://github.com/Nils-TUD/Escape/blob/master/doc/x86.ioports.txt
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Port {
  PIC1Command             = 0x0020,
  PIC1Data                = 0x0021,
  PIC2Command             = 0x00A0,
  PIC2Data                = 0x00A1,
  DebugOut                = 0x00E9
}

pub trait PortNumber {
  fn as_u16(&self) -> u16;
}

impl PortNumber for Port {
  fn as_u16(&self) -> u16 {
    (*self) as u16
  }
}

impl PortNumber for u16 {
  fn as_u16(&self) -> u16 {
    *self
  }
}

#[allow(dead_code)]
pub unsafe fn inb(port: &dyn PortNumber) -> u8 {
  x86_64::instructions::port::Port::new(port.as_u16()).read()
}

#[allow(dead_code)]
pub unsafe fn inw(port: &dyn PortNumber) -> u16 {
  x86_64::instructions::port::Port::new(port.as_u16()).read()
}

#[allow(dead_code)]
pub unsafe fn inl(port: &dyn PortNumber) -> u32 {
  x86_64::instructions::port::Port::new(port.as_u16()).read()
}

#[allow(dead_code)]
pub unsafe fn outb(port: &dyn PortNumber, data: u8) {
  x86_64::instructions::port::Port::new(port.as_u16()).write(data);
}

#[allow(dead_code)]
pub unsafe fn outw(port: &dyn PortNumber, data: u16) {
  x86_64::instructions::port::Port::new(port.as_u16()).write(data);
}

#[allow(dead_code)]
pub unsafe fn outl(port: &dyn PortNumber, data: u32) {
  x86_64::instructions::port::Port::new(port.as_u16()).write(data);
}

#[allow(dead_code)]
pub fn io_wait() {
  unsafe { outb(&0x80, 0); }
}