// prevent linking standard library
#![no_std]
// omit default entry point (main) in favor of custom entry point
#![no_main]

use core::panic::PanicInfo;

// entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

// handle panic (divergent)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
