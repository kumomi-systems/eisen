#![no_std]
#![no_main]

mod panic;

#[unsafe(no_mangle)]
extern "C" fn kentry() -> ! {
  loop {}
}