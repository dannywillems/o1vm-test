#![no_std]
#![no_main]

use core::panic::PanicInfo;

const MAX_ITERATIONS: u32 = 16384;

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
pub extern "C" fn _start() -> () {
    let mut x = 1;
    loop {
        x += 1;
        let _res1 = fibonacci(x);
        if x == MAX_ITERATIONS {
            break;
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
