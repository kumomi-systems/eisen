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

#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec, format};

use crc::{self, CRC_32_ISO_HDLC};

pub const STUB_SIZE:        usize = size_of::<Stub>();
pub const PMBR_SIZE:        usize = size_of::<PMBR>();
pub const BOOT_INFO_SIZE:   usize = size_of::<BootInfo>();
pub const BOOT_INFO_OFFSET: usize = PMBR_SIZE;

#[repr(C, packed)]
pub struct Stub {
  pmbr: PMBR,
  bi:   BootInfo
}

pub type StubBuffer = [u8; STUB_SIZE];
pub const fn new_stub_buffer() -> StubBuffer {
  [0 as u8; STUB_SIZE]
}

impl From<StubBuffer> for Stub {
  fn from(value: StubBuffer) -> Self {
    unsafe {
      core::mem::transmute::<StubBuffer, Self>(value)
    }
  }
}

pub type PMBR = [u8; 0x200];

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct BootInfo {
  magic_start:    [u8; 6],
  header_version: u8,
  kernel_type:    KernelType,
  header_size:    u16,
  res_t_0:        [u8; 6],
  install_uuid:   [u8; 16],
  section_size:   u8,
  res_t_1:        [u8; 3],
  build_year:     u16,
  build_month:    u8,
  build_day:      u8,
  major_version:  u8,
  minor_version:  u8,
  patch_version:  u8,
  version_flags:  u8,
  res_t_2:        [u8; 4],
  version_name:   [u8; 16],

  uuid0:          u16,
  res_s0_0:       [u8; 6],
  kentry:         core::ptr::NonNull<extern "C" fn () -> u8>,
  kargs:          core::ptr::NonNull<()>,
  ksysinfo:       core::ptr::NonNull<()>,
  stub_end:       u64,
  kernel_vma:     u64,
  kernel_size:    u64,
  stack_top:      u64,

  uuid1:          u16,
  pad_s1:         [u8; 62],

  uuid2:          u16,
  pad_s2:         [u8; 62],

  uuid3:          u16,
  pad_s3:         [u8; 62],

  uuid4:          u16,
  pad_s4:         [u8; 62],

  uuid5:          u16,
  pad_s5:         [u8; 62],

  uuid6:          u16,
  res_s6_0:       [u8; 48],
  uuid7:          u16,
  checksum:       u32,
  magic_end:      [u8; 8]
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum KernelType {
  Flat  = 0,
  Elf   = 1
}

#[derive(Debug)]
pub enum BootInfoInvalidError {
  MagicStart,
  MagicEnd,
  Unsigned,
  EisenUUID,
  Hash
}

macro_rules! expose_member {
  ($member:ident, $type:ty) => {
    pub fn $member(&self) -> $type {
      self.$member
    }
  };
}

impl BootInfo {
  const MAGIC_START:  [u8; 6]         = *b"Eisen\0";
  const MAGIC_END:    [u8; 8]         = *b"InfoEnd\x1A";
  const EISEN_UUID:   [u8; 16]        =   // 94595C96-BD12-40E1-A7FB-61C50F3DCA9A
    [ 0x94, 0x59, 0x5C, 0x96,             // 94595C96
      0xBD, 0x12,                         // BD12
      0x40, 0xE1,                         // 40E1
      0xA7, 0xFB,                         // A7FB
      0x61, 0xC5, 0x0F, 0x3D, 0xCA, 0x9A  // 61C50F3DCA9A
    ];

  pub const VERSION_FLAG_ALPHA: u8  = 0b00000001;
  pub const VERSION_FLAG_BETA:  u8  = 0b00000010;
  pub const VERSION_FLAG_RC:    u8  = 0b00000100;
  pub const VERSION_FLAG_XPER:  u8  = 0b00001000;
  
  pub const CRC32_HASHER: crc::Crc<u32>  = crc::Crc::<u32>::new(&CRC_32_ISO_HDLC);

  expose_member!(header_version, u8);
  expose_member!(kernel_type,    KernelType);
  expose_member!(header_size,    u16);
  expose_member!(install_uuid,   [u8; 16]);
  expose_member!(section_size,   u8);
  expose_member!(build_year,     u16);
  expose_member!(build_month,    u8);
  expose_member!(build_day,      u8);
  expose_member!(major_version,  u8);
  expose_member!(minor_version,  u8);
  expose_member!(patch_version,  u8);
  expose_member!(version_flags,  u8);
  expose_member!(version_name,   [u8; 16]);
  expose_member!(kentry,         core::ptr::NonNull<extern "C" fn () -> u8>);
  expose_member!(kargs,          core::ptr::NonNull<()>);
  expose_member!(ksysinfo,       core::ptr::NonNull<()>);
  expose_member!(stub_end,       u64);
  expose_member!(kernel_vma,     u64);
  expose_member!(kernel_size,    u64);
  expose_member!(stack_top,      u64);
  expose_member!(checksum,       u32);

  pub fn parse(buffer: &Stub) -> Result<Self, BootInfoInvalidError> {
    let mut ret = unsafe { Self::from(*(buffer as *const Stub as *const StubBuffer)) };

    match ret.validate() {
      None        => { Ok(ret) }
      Some(some)  => { Err(some) }
    }
  }

  fn validate(&mut self) -> Option<BootInfoInvalidError> {
    let mut patient = self.clone();

    // Check start magic number
    if patient.magic_start != Self::MAGIC_START {
      return Some(BootInfoInvalidError::MagicStart);
    }
  
    // Check end magic number
    if patient.magic_end != Self::MAGIC_END {
      return Some(BootInfoInvalidError::MagicEnd);
    }

    // Check signature
    if patient.install_uuid == [0; 16] {
      return Some(BootInfoInvalidError::Unsigned);
    }

    // Check Eisen UUID
    let mut uuid_buffer: [u8; 16] = [0; 16];
    unsafe {
      *(uuid_buffer.as_mut_ptr().add(0x00) as *mut u16) = self.uuid0.to_be();
      *(uuid_buffer.as_mut_ptr().add(0x02) as *mut u16) = self.uuid1.to_be();
      *(uuid_buffer.as_mut_ptr().add(0x04) as *mut u16) = self.uuid2.to_be();
      *(uuid_buffer.as_mut_ptr().add(0x06) as *mut u16) = self.uuid3.to_be();
      *(uuid_buffer.as_mut_ptr().add(0x08) as *mut u16) = self.uuid4.to_be();
      *(uuid_buffer.as_mut_ptr().add(0x0A) as *mut u16) = self.uuid5.to_be();
      *(uuid_buffer.as_mut_ptr().add(0x0C) as *mut u16) = self.uuid6.to_be();
      *(uuid_buffer.as_mut_ptr().add(0x0E) as *mut u16) = self.uuid7.to_be();
    }
    if uuid_buffer != Self::EISEN_UUID {
      return Some(BootInfoInvalidError::EisenUUID);
    }

    // Compare CRC32 checksums
    patient.checksum          = 0;
    if Self::CRC32_HASHER.checksum(
      &unsafe { core::mem::transmute::<Self, [u8; size_of::<BootInfo>()]>(patient) }
    ) != self.checksum {
      return Some(BootInfoInvalidError::Hash);
    }

    None
  }

  #[cfg(feature = "alloc")]
  pub fn version_info(&self) -> String {
    let mut version_labels: Vec<&str> = Vec::new();

    if self.version_flags & BootInfo::VERSION_FLAG_ALPHA != 0 {
      version_labels.push("alpha");
    }
    if self.version_flags & BootInfo::VERSION_FLAG_BETA != 0 {
      version_labels.push("beta");
    }
    if self.version_flags & BootInfo::VERSION_FLAG_RC != 0 {
      version_labels.push("rc");
    }
    if self.version_flags & BootInfo::VERSION_FLAG_XPER != 0 {
      version_labels.push("experimental");
    }

    let version_labels_pretty = if version_labels.is_empty() { "" } else { 
      &format!("[ {} ]", version_labels.join(" "))
    };

    format!(
      "{}.{}.{} \"{}\" {}",
      self.major_version,
      self.minor_version,
      self.patch_version,
      String::from_utf8(self.version_name.into()).unwrap().trim_end_matches('\0'),
      version_labels_pretty
    )
  }

  #[cfg(feature = "alloc")]
  pub fn date(&self) -> String {
    let year  = self.build_year;
    let month = self.build_month;
    let day   = self.build_day;
    format!("{:#04}-{:#02}-{:#02}", year, month, day)
  }

  #[cfg(feature = "alloc")]
  pub fn uuid_pretty(&self) -> String {
    format!(
      "{:X}{:X}{:X}{:X}-{:X}{:X}-{:X}{:X}-{:X}{:X}-{:X}{:X}{:X}{:X}{:X}{:X}",
      self.install_uuid.get(0x0).unwrap(),
      self.install_uuid.get(0x1).unwrap(),
      self.install_uuid.get(0x2).unwrap(),
      self.install_uuid.get(0x3).unwrap(),
      self.install_uuid.get(0x4).unwrap(),
      self.install_uuid.get(0x5).unwrap(),
      self.install_uuid.get(0x6).unwrap(),
      self.install_uuid.get(0x7).unwrap(),
      self.install_uuid.get(0x8).unwrap(),
      self.install_uuid.get(0x9).unwrap(),
      self.install_uuid.get(0xa).unwrap(),
      self.install_uuid.get(0xb).unwrap(),
      self.install_uuid.get(0xc).unwrap(),
      self.install_uuid.get(0xd).unwrap(),
      self.install_uuid.get(0xe).unwrap(),
      self.install_uuid.get(0xf).unwrap()
    )
  }

  #[cfg(feature = "alloc")]
  pub fn kernel_size_pretty(&self) -> String {
    let kernel_size = self.kernel_size;
    let kernel_size_adjusted: u64;
    let kernel_size_unit: char;

    match kernel_size {
      0..0x400 => {
        kernel_size_adjusted  = kernel_size;
        kernel_size_unit      = 'B';
      },
      0x400..0x100000 => {
        kernel_size_adjusted  = kernel_size / 0x400;
        kernel_size_unit      = 'K';
      },
      0x100000..0x40000000 => {
        kernel_size_adjusted  = kernel_size / 0x100000;
        kernel_size_unit      = 'M';
      }
      0x40000000..=u64::MAX => {
        kernel_size_adjusted  = kernel_size / 0x40000000;
        kernel_size_unit      = 'G';
      }
    }

    format!("{}{}", kernel_size_adjusted, kernel_size_unit)
  }
}

impl From<StubBuffer> for BootInfo {
  fn from(value: StubBuffer) -> Self {
    unsafe {
      *core::mem::transmute::<*const u8, *const Self>((&value as *const u8).add(PMBR_SIZE))
    }
  }
}