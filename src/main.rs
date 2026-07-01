#![no_std]
#![no_main]

///---includes---///
mod vga;
use core::panic::PanicInfo;

#[panic_handler]
fn kpanic(_info: &PanicInfo) -> ! {
    loop {}
}

///---Main-Function---///
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}
