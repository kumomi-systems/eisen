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

use eisen_kernel_stub::*;
use uefi::{CStr16, cstr16};
use uefi::proto::media::file::{FileMode, RegularFile};

use crate::log;
use crate::logger::*;

const KERNEL_FILE_PATH: &CStr16 = cstr16!("eisen\\kernel");

pub fn read_file() -> RegularFile {
  match crate::fs::read_file(KERNEL_FILE_PATH, FileMode::Read) {
    Some(some)  => { some }
    None        => { panic!("Failed to locate kernel (should be located at {})", KERNEL_FILE_PATH) }
  }
}

pub fn read_stub_buffer(file: &mut RegularFile) -> StubBuffer {
  let mut kernel_stub_buf = new_stub_buffer();
	match file.read(&mut kernel_stub_buf) {
		Ok(ok)		=> {
			if ok < STUB_SIZE {
				panic!("Read {} bytes of kernel stub, expected {}", ok, STUB_SIZE)
			}
      kernel_stub_buf
		}
		Err(err)	=> { panic!("Reading kernel stub: {}", err) }
	}
}

pub fn get_boot_info(buffer: &StubBuffer) -> BootInfo {
	match BootInfo::parse(&Stub::from(*buffer)) {
		Ok(ok) 		=> { ok }
		Err(err)	=> { panic!("Parsing kernel stub: {:?}", err) }
	}
}

pub fn print_boot_info(bi: &BootInfo) {
	log!(LOGLEVEL_NORMAL, r#"--- Product Information ---
    Version:      {}
    UUID:         {}
    Date Built:   {}
    Size:         {}
    Checksum:     {:08X}"#,
	bi.version_info(),
	bi.uuid_pretty(),
	bi.date(),
	bi.kernel_size_pretty(),
	bi.checksum());
	log!(LOGLEVEL_VERBOSE, r#"--- Kernel Information ---
    Kernel Type:  {:?}
    kentry:       {:#018X}
    kargs:        {:#018X}
    ksysinfo:     {:#018X}
    stack:        {:#018X}
    VMA:          {:#018X}"#,
	bi.kernel_type(),
	bi.kentry().addr(),
	bi.kargs().addr(),
	bi.ksysinfo().addr(),
	bi.stack_top(),
	bi.kernel_vma()
	)
}