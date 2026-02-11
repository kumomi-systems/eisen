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

use uefi::CStr16;
use uefi::boot::ScopedProtocol;
use uefi::proto::media::file::{Directory, File, FileAttribute, FileMode, RegularFile};
use uefi::proto::media::fs::SimpleFileSystem;

pub fn read_file(path: &CStr16, mode: FileMode) -> Option<RegularFile> {
	let mut bootfs: ScopedProtocol<SimpleFileSystem>;
  match uefi::boot::get_image_file_system(uefi::boot::image_handle()) {
    Ok(ok)    => {
      bootfs = ok;
    }
    Err(err)  => { panic!("Opening bootfs: {}", err); }
  }

  let mut bootdir: Directory;
  match bootfs.open_volume() {
    Ok(ok)    => { bootdir = ok; }
    Err(err)  => { panic!("Opening root directory: {}", err); }
  }

  match bootdir.open(
    path,
    mode,
    FileAttribute::empty()
  ) {
    Ok(ok)    => {
      if ok.is_directory().unwrap() { None }
      else { Some(ok.into_regular_file().unwrap()) }
    }
    Err(_)    => { None }
  }
}