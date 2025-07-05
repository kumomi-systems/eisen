use crate::kdata::ksysinfo;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".kentry")]

extern "C" fn _kentry() -> ! {
  for x in 0..ksysinfo().graphicsinfo.framebuffer_size {
    unsafe {
      *(ksysinfo().graphicsinfo.framebuffer_base.add(x)) = 0x20;
    }
  }

  loop {}
}