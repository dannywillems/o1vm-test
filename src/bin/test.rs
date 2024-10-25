// src/main.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;

const FIBONACCI_N: u32 = 10;

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

// Only for debugging
#[no_mangle]
pub fn fibonacci_iterative(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 0..n {
        c = a + b;
        a = b;
        b = c;
    }
    return a;
}

// Entry point of our program
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut x = FIBONACCI_N;
    loop {
        x += 1;
        let _ = fibonacci(x);
        let _ = fibonacci_iterative(x);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
