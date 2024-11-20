#![no_std]
#![no_main]

use core::panic::PanicInfo;

const N: u32 = 100;

// Only for debugging
#[no_mangle]
pub fn is_prime_naive(n: u32) -> bool {
    if n == 0 || n == 1 {
        return false;
    } else if n == 2 {
        return true;
    } else {
        for i in 2..n {
            if n % i == 0 {
                return false;
            }
        }
        return true;
    }
}

// Entry point of our program
#[no_mangle]
pub fn _start() {
    let mut n = 1;
    let mut prime_count = 0;
    while n <= N {
        if is_prime_naive(n) {
            prime_count += 1;
        }
        n += 1;
    }
    if prime_count != 25 {
        o1vm_stdlib::syscall::exit_panic();
    } else {
        o1vm_stdlib::syscall::exit_success();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        o1vm_stdlib::syscall::exit_panic();
    }
}
