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

mod config;
mod fs;
mod kernel;
mod logger;

use uefi::boot::{MemoryType, PAGE_SIZE, allocate_pages};
use uefi::prelude::*;
use uefi::proto::{console::text::Output};

use crate::logger::LOGLEVEL_VERBOSE;

#[entry]
fn main() -> Status {
	// Basic init
	uefi::helpers::init().unwrap();
	let _ = uefi::system::with_stdout(Output::clear);
	uefi::println!("Eisen NativeBoot version {}", env!("CARGO_PKG_VERSION"));

	config::load_config();
	let mut kernel	= kernel::read_file();
	let bi					= kernel::get_boot_info(&kernel::read_stub_buffer(&mut kernel));
	kernel::print_boot_info(&bi);
	
	let kernel_load_ptr = allocate_pages(
		boot::AllocateType::Address(bi.kernel_vma()),
		boot::MemoryType::LOADER_CODE,
		bi.kernel_size() as usize / PAGE_SIZE
	).unwrap();
	log!(LOGLEVEL_VERBOSE, "Allocated {} for the kernel at phys addr {:#018X}", bi.kernel_size_pretty(), kernel_load_ptr.addr());
	
	let kernel_buf = crate::fs::read_contents(&mut kernel);
	let kernel_load_buf = unsafe {
		core::slice::from_raw_parts_mut(
			kernel_load_ptr.as_ptr(),
			kernel_buf.len()
		)
	};
	kernel_load_buf.copy_from_slice(&kernel_buf);
	log!(LOGLEVEL_VERBOSE, "Loaded Kernel");

	unsafe {
		let cmdline_bytes = crate::config::CONFIG.get().unwrap().cmdline.as_bytes();
		if cmdline_bytes.len() > eisen_kernel_data::kargs::KARGS_SIZE {
			panic!("cmdline exceeds maximum length: must be no greater than {} bytes", eisen_kernel_data::kargs::KARGS_SIZE);
		}
		let kargs = core::slice::from_raw_parts_mut(
			bi.kargs().addr().get() as *mut u8,
			cmdline_bytes.len()
		);
		kargs.copy_from_slice(cmdline_bytes);

		let kentry = bi.kentry().addr().get();
		log!(LOGLEVEL_VERBOSE, "Jumping to kernel at addr {:#018X}", kentry);
		let _ = uefi::boot::exit_boot_services(Some(MemoryType::LOADER_DATA));
		let entry_fn: extern "C" fn() -> ! = core::mem::transmute(kentry);
		entry_fn();
	}

	loop {}
	Status::SUCCESS
}