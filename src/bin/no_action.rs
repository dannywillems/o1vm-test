#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Entry point of our program
#[no_mangle]
pub fn _start() {
    o1vm_stdlib::syscall::exit_success();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        o1vm_stdlib::syscall::exit_panic();
    }
}
