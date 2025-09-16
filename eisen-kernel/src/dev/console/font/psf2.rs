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

use core::marker::PhantomData;

#[derive(Clone, Copy, Default)]
#[repr(C, packed)]
struct PSF2Header {
  magic:            [u8; 4],
  version:          u32,
  header_size:      u32,
  flags:            u32,
  glyph_count:      u32,
  bytes_per_glyph:  u32,
  height:           u32,
  width:            u32
}

impl PSF2Header {
  const MAGIC: [u8; 4] = [0x72, 0xB5, 0x4A, 0x86];

  const FLAG_HAS_UNICODE_TABLE: u32 = 0x00000001;

  fn validate(&self) -> bool {
    self.magic == Self::MAGIC
  }

  fn supports_unicode(&self) -> bool {
    self.flags & Self::FLAG_HAS_UNICODE_TABLE != 0
  }
}

#[derive(Default)]
pub struct PSF2 {
  header: PSF2Header,
}

impl<'a, 'b: 'a> super::FontBuilder for PSF2 {
  fn init(&mut self, data: &[u8]) -> Result<(), ()> {
    self.header = unsafe { *(data.as_ptr() as *const PSF2Header) };
    Ok(())
  }

  fn dimensions(&self) -> (u8, u8) {
    (self.header.width as u8, self.header.height as u8)
  }

  fn length(&self) -> usize {
    self.header.glyph_count as usize
  }

  fn get_glyph(&self, data: &[u8], c: char) -> Option<super::Glyph> {
    if !c.is_ascii() && !self.header.supports_unicode() { return None; }

    unsafe {
      let offset = self.header.header_size + (self.header.bytes_per_glyph * c as u32);
      if offset >= self.header.header_size + (self.header.bytes_per_glyph * self.header.glyph_count) {
        return None;
      }
      let glyph = core::slice::from_raw_parts(
        data.as_ptr().add(offset as usize),
        self.header.bytes_per_glyph as usize
      );
      Some(super::Glyph(glyph))
    }
  }

  fn get_glyphs(&self, data: &[u8]) -> &[super::Glyph] {
    // let mut c = 0;
    // let mut ret = core::slice::from
    // loop {
    //   if char::from_u32(c).is_none() {
    //     return ret;
    //   }
    //   match self.get_glyph(data, char::from_u32(c).unwrap()) {
    //     Some(glyph) => {
    //       ret = [ret, [glyph]].concat()
    //     }
    //     None => {
    //       return ret;
    //     }
    //   }
    // }
    todo!();
  }
}