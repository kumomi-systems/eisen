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

extern crate alloc;

use uefi::CStr16;
use uefi::boot::ScopedProtocol;
use uefi::proto::media::file::{Directory, File, FileAttribute, FileInfo, FileMode, RegularFile};
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

pub fn read_contents(file: &mut RegularFile) -> alloc::vec::Vec<u8> {
  let file_info: alloc::boxed::Box<FileInfo>;
  match file.get_boxed_info() {
    Ok(ok)      => { file_info = ok; }
    Err(err)    => { panic!("Failed to determine file size: {}", err) }
  }

  file.set_position(0);
  let file_size = file_info.file_size() as usize;
  let mut file_buffer = alloc::vec![0; file_size];
  match file.read(&mut file_buffer) {
    Ok(ok)      => {
      if ok != file_size {
        panic!("Failed to read file")
      }
    }
    Err(err)    => { panic!("Failed to read file: {}", err) }
  }

  return file_buffer
}