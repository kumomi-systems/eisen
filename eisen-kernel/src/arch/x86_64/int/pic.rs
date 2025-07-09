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

use crate::{arch::x86_64::ports::{inb, io_wait, outb, Port}, debugln};

pub unsafe fn mask_irq(line: u8) {
  let mut irqline = line;
  let port = if line < 8 {
    Port::PIC1Data
  } else {
    irqline -= 8;
    Port::PIC2Data
  };

  let value = inb(&port) | (1 << irqline);
  outb(&port, value);
}

pub unsafe fn unmask_irq(line: u8) {
  let mut irqline = line;
  let port = if line < 8 {
    Port::PIC1Data
  } else {
    irqline -= 8;
    Port::PIC2Data
  };

  let value = inb(&port) & !(1 << irqline);
  outb(&port, value);
}

pub unsafe fn remap(pic1_offset: u8, pic2_offset: u8) {
  outb(&Port::PIC1Command, 0x11);     io_wait();
  outb(&Port::PIC2Command, 0x11);     io_wait();

  outb(&Port::PIC1Data, pic1_offset); io_wait();
  outb(&Port::PIC2Data, pic2_offset); io_wait();

  outb(&Port::PIC1Data, 0x04);        io_wait();
  outb(&Port::PIC2Data, 0x02);        io_wait();

  outb(&Port::PIC1Data, 0x01);        io_wait();
  outb(&Port::PIC2Data, 0x01);        io_wait();

  outb(&Port::PIC1Data, 0x00);        io_wait();
  outb(&Port::PIC2Data, 0x00);        io_wait();

  debugln!("Remapped PIC");
}

pub unsafe fn acknowledge(vector: u8) {
  let port = if vector < 8 { Port::PIC1Command } else { Port::PIC2Command };
  outb(&port, 0x20);
}

pub unsafe fn enable() {
  outb(&Port::PIC1Data, 0x00);
  outb(&Port::PIC2Data, 0x00);

  debugln!("Enabled PIC");
}

pub unsafe fn disable() {
  outb(&Port::PIC1Data, 0xFF);
  outb(&Port::PIC2Data, 0xFF);

  debugln!("Disabled PIC");
}