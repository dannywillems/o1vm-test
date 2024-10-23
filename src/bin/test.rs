#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Note that this should include some o1vm infos
    loop {}
}

#[no_mangle]
pub fn main() {
    let _ = 3 + 4;
}
