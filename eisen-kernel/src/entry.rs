#[unsafe(no_mangle)]
extern "C" fn kentry() -> ! {
  loop {}
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".kargs")]
#[used]
pub static kargs: [u8; 0x400] = [0; 0x400];