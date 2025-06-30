#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::{string::String, vec::Vec, format};

use crc::{self, CRC_32_ISO_HDLC};
use uuid::{uuid, Uuid};

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct BootInfo {
  pub magic_start:    [u8; 6],
      res_t_0:        [u8; 10],
  pub install_uuid:   [u8; 16],
  pub section_size:   u8,
      res_t_1:        [u8; 3],
  pub build_year:     u16,
  pub build_month:    u8,
  pub build_day:      u8,
  pub major_version:  u8,
  pub minor_version:  u8,
  pub patch_version:  u8,
  pub version_flags:  u8,
      res_t_2:        [u8; 4],
  pub version_name:   [u8; 16],

  pub uuid0:          u16,
      res_s0_0:       [u8; 6],
  pub kentry:         core::ptr::NonNull<extern "C" fn() -> !>,
  pub kargs:          core::ptr::NonNull<&'static [u8; 0x400]>,
  pub ksysinfo:       core::ptr::NonNull<crate::boot::sysinfo::SysInfo>,
  pub kloadaddr:      u64,
  pub kernel_offset:  u64,
  pub kernel_size:    u64,
      pad_s0:         [u8; 8],

  pub uuid1:          u16,
      pad_s1:         [u8; 62],

  pub uuid2:          u16,
      pad_s2:         [u8; 62],

  pub uuid3:          u16,
      pad_s3:         [u8; 62],

  pub uuid4:          u16,
      pad_s4:         [u8; 62],

  pub uuid5:          u16,
      pad_s5:         [u8; 62],

  pub uuid6:          u16,
      res_s6_0:       [u8; 48],
  pub uuid7:          u16,
  pub checksum:       u32,
  pub magic_end:      [u8; 8]
}

pub type BootInfoByteArray = [u8; size_of::<BootInfo>()];

#[derive(Debug)]
pub enum BootInfoInvalidError {
  MagicStart,
  MagicEnd,
  Unsigned,
  EisenUUID,
  Hash
}

impl BootInfo {
  const MAGIC_START:  [u8; 6]         = *b"Eisen\0";
  const MAGIC_END:    [u8; 8]         = *b"InfoEnd\x1A";
  const EISEN_UUID:   Uuid            = uuid!("94595C96-BD12-40E1-A7FB-61C50F3DCA9A");

  pub const VERSION_FLAG_ALPHA: u8  = 0b00000001;
  pub const VERSION_FLAG_BETA:  u8  = 0b00000010;
  pub const VERSION_FLAG_RC:    u8  = 0b00000100;
  pub const VERSION_FLAG_XPER:  u8  = 0b00001000;
  
  pub const CRC32_HASHER: crc::Crc<u32>  = crc::Crc::<u32>::new(&CRC_32_ISO_HDLC);

  pub fn parse(buffer: BootInfoByteArray) -> Result<Self, BootInfoInvalidError> {
    // Read from buffer
    let mut ret = unsafe {
      core::mem::transmute::<BootInfoByteArray, Self>(buffer)
    };

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
    if Uuid::from_bytes(patient.install_uuid) == Uuid::nil() {
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
    if Uuid::from_bytes(uuid_buffer) != Self::EISEN_UUID {
      return Some(BootInfoInvalidError::EisenUUID);
    }

    // Compare CRC32 checksums
    patient.checksum          = 0;
    if Self::CRC32_HASHER.checksum(
      &unsafe { core::mem::transmute::<Self, BootInfoByteArray>(patient) }
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