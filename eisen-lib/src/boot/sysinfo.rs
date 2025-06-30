use core::ffi::c_void;

#[repr(C, packed)]
pub struct SysInfo {
  pub acpi_addr:    *const c_void,
  pub acpi_entries: usize
}

impl SysInfo {
  pub const fn new() -> Self {
    Self {
      acpi_addr:    0 as *const c_void,
      acpi_entries: 0,
    }
  }
}