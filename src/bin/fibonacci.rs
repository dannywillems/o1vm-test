#![no_std]
#![no_main]

use core::panic::PanicInfo;

const MAX_ITERATIONS: u32 = 30;

// Only for debugging
#[no_mangle]
pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

// Entry point of our program
#[no_mangle]
pub fn _start() {
    let mut x = 1;
    while x <= MAX_ITERATIONS {
        let _res1 = fibonacci(x);
        x += 1;
    }
    o1vm_stdlib::syscall::exit_success();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
