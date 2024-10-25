// src/main.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut x = 0;
    loop {
        x += 1;
        let _ = fibonacci(x);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
