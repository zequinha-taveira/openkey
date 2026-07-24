//! OpenKey RP2350 Target Firmware (`no_std`)

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// Ponto de entrada do firmware RP2350
#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}
