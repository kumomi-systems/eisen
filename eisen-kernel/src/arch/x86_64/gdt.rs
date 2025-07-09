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

use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::*;
use x86_64::structures::{gdt::{Descriptor, GlobalDescriptorTable}, tss::TaskStateSegment};

use crate::debugln;

static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();
static mut TSS: TaskStateSegment      = TaskStateSegment::new();

#[allow(static_mut_refs)]
pub unsafe fn load_gdt() {
  let kernel_code_segment = GDT.append(Descriptor::kernel_code_segment());
  let kernel_data_segment = GDT.append(Descriptor::kernel_data_segment());
  let tss_segment         = GDT.append(Descriptor::tss_segment(&TSS));
  GDT.load();
  debugln!("Loaded GDT");

  CS::set_reg(kernel_code_segment);
  DS::set_reg(kernel_data_segment);
  ES::set_reg(kernel_data_segment);
  FS::set_reg(kernel_data_segment);
  GS::set_reg(kernel_data_segment);
  SS::set_reg(kernel_data_segment);
  debugln!("Reloaded segment registers");

  load_tss(tss_segment);
  debugln!("Loaded TSS");
}