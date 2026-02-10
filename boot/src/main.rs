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

#![no_main]
#![no_std]

mod fs;

use uefi::{CStr16, prelude::*};
use uefi::proto::{console::text::Output};

const KERNEL_FILE_PATH: &CStr16 = cstr16!("eisen\\kernel");

#[entry]
fn main() -> Status {
	// Basic init
	uefi::helpers::init().unwrap();
	let _ = uefi::system::with_stdout(Output::clear);
	uefi::println!("Eisen NativeBoot version {}", env!("CARGO_PKG_VERSION"));

	let mut kernel = fs::get_kernel_file();
	let mut kernel_hdr: [u8; 0x400] = [0; 0x400];

	let _ = kernel.read(&mut kernel_hdr).unwrap();
	uefi::println!("{:X?}", kernel_hdr);

	loop {}
	Status::SUCCESS
}