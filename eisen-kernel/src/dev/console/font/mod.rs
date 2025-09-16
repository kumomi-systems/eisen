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

use spin::{Lazy, RwLock};

mod charset;
mod psf2;

const DEFAULT_FONT: &[u8] = include_bytes!("fonts/Tamsyn8x16r.psf");

static CURRENT_FONT: Lazy<RwLock<Font>> = Lazy::new(|| {
  let mut psf2 = psf2::PSF2::default();
  spin::RwLock::new(Font::parse(DEFAULT_FONT, &mut psf2).unwrap())
});

trait FontBuilder {
  fn init(&mut self, data: &[u8]) -> Result<(), ()>;
  fn dimensions(&self) -> (u8, u8);
  fn length(&self) -> usize;
  fn get_glyph(&self, data: &[u8], c: char) -> Option<Glyph>;
  fn get_glyphs(&self, data: &[u8]) -> &[Glyph];
}

struct Glyph<'a>(&'a [u8]);
struct Font<'a> {
  pub height: u8,
  pub width:  u8,
  values:     &'a [Glyph<'a>]
}

impl Font<'_> {
  pub fn parse(data: &[u8], builder: &mut dyn FontBuilder) -> Option<Self> {
    if builder.init(data).is_err() { return None; }
    
    let mut ret = Self {
      width:  builder.dimensions().0,
      height: builder.dimensions().1,
      values: &[]
    };

    Some(ret)
  }
}