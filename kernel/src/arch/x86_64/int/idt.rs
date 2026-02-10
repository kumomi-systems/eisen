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

use x86_64::registers::segmentation::{Segment, CS};
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::VirtAddr;

use crate::debugln;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

unsafe extern "C" {
  static ISR_TABLE: [*const core::ffi::c_void; 256];
}

#[allow(static_mut_refs)]
pub unsafe fn load_idt() {
  IDT.divide_error                .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x00]));
  IDT.debug                       .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x01]));
  IDT.non_maskable_interrupt      .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x02]));
  IDT.breakpoint                  .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x03]));
  IDT.overflow                    .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x04]));
  IDT.bound_range_exceeded        .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x05]));
  IDT.invalid_opcode              .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x06]));
  IDT.device_not_available        .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x07]));
  IDT.double_fault                .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x08]));
  IDT.invalid_tss                 .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x0A]));
  IDT.segment_not_present         .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x0B]));
  IDT.stack_segment_fault         .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x0C]));
  IDT.general_protection_fault    .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x0D]));
  IDT.page_fault                  .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x0E]));
  IDT.x87_floating_point          .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x10]));
  IDT.alignment_check             .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x11]));
  IDT.machine_check               .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x12]));
  IDT.simd_floating_point         .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x13]));
  IDT.virtualization              .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x14]));
  IDT.cp_protection_exception     .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x15]));
  IDT.hv_injection_exception      .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x1C]));
  IDT.vmm_communication_exception .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x1D]));
  IDT.security_exception          .set_handler_addr(VirtAddr::from_ptr(ISR_TABLE[0x1E]));
  debugln!("Installed CPU exception ISRs");

  for vector in 32..=255 {
    set_idt_descriptor(
      vector,
      VirtAddr::from_ptr(ISR_TABLE[vector as usize]),
      0x8E
    );
  }
  debugln!("Installed external interrupt ISRs");
  
  IDT.load();
  debugln!("Loaded IDT");
}

#[allow(static_mut_refs)]
pub unsafe fn set_idt_descriptor(vector: u8, isr: VirtAddr, attrs: u8) {
  let mut entry = IDT[vector];
  let entry_options = entry.set_handler_addr(isr);
  
  entry_options.set_present(attrs & 0x80 != 0);
  entry_options.set_privilege_level(match attrs & 0x60 >> 5 {
    0 => x86_64::PrivilegeLevel::Ring0,
    1 => x86_64::PrivilegeLevel::Ring1,
    2 => x86_64::PrivilegeLevel::Ring2,
    3 => x86_64::PrivilegeLevel::Ring3,
    _ => unreachable!()
  });
  entry_options.set_code_selector(CS::get_reg());
}