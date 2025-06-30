use eisen_lib::boot::sysinfo::SysInfo;

#[unsafe(no_mangle)]
extern "C" fn _kentry() -> ! {
  loop {}
}

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".kdata")]
#[used]
static _kargs: [u8; 0x400] = [0; 0x400];

pub fn kargs() -> &'static str {
  core::str::from_utf8(&_kargs).unwrap()
}

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".kdata")]
#[used]
static _ksysinfo: [u8; size_of::<SysInfo>()] = [0; size_of::<SysInfo>()];

pub unsafe fn ksysinfo() -> &'static SysInfo {
  unsafe {
    (&_ksysinfo as *const [u8; size_of::<SysInfo>()] as *const SysInfo).as_ref().unwrap()
  }
}