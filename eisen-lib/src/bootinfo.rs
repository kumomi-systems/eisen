use crc::{self, CRC_32_ISO_HDLC};
use uuid::{uuid, Uuid};

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct BootInfo {
  pub magic_start:    [u8; 6],
      reserved0:      [u8; 10],
  pub os_uuid:        [u8; 16],
  pub section_size:   u8,
  pub major_version:  u8,
  pub minor_version:  u8,
  pub patch_version:  u8,
  pub version_flags:  u8,
      section_pad0:   [u8; 27],

  pub uuid0:          u16,
  pub uuid1:          u16,
  pub reserved1:      [u8; 4],
  pub kentry:         core::ptr::NonNull<extern "C" fn() -> !>,
  pub kargs:          core::ptr::NonNull<&'static [u8]>,
      section_pad1:   [u8; 40],

  pub uuid2:          u16,
      section_pad2:   [u8; 62],

  pub uuid3:          u16,
      section_pad3:   [u8; 62],

  pub uuid4:          u16,
      section_pad4:   [u8; 62],

  pub uuid5:          u16,
  pub uuid6:          u16,
  pub uuid7:          u16,
      section_pad5:   [u8; 154],

  pub checksum:       u32,
      reserved2:      [u8; 20],
  pub magic_end:      [u8; 8]
}

pub type BootInfoByteArray = [u8; size_of::<BootInfo>()];

#[derive(Debug)]
pub enum BootInfoInvalidError {
  MagicStart,
  MagicEnd,
  EisenUUID,
  Hash
}

impl BootInfo {
  const MAGIC_START:  [u8; 6]         = *b"Eisen\0";
  const MAGIC_END:    [u8; 8]         = *b"InfoEnd\x1A";
  const EISEN_UUID:   Uuid            = uuid!("94595C96-BD12-40E1-A7FB-61C50F3DCA9A");
  
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

    // Check Eisen UUID
    if Uuid::from_bytes(
      [
        self.uuid0.to_be_bytes(),
        self.uuid1.to_be_bytes(),
        self.uuid2.to_be_bytes(),
        self.uuid3.to_be_bytes(),
        self.uuid4.to_be_bytes(),
        self.uuid5.to_be_bytes(),
        self.uuid6.to_be_bytes(),
        self.uuid7.to_be_bytes()
      ].concat().try_into().unwrap()
    ) != Self::EISEN_UUID {
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
}