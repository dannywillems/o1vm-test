#![no_std]
#![no_main]

use core::arch::asm;
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
    // Our convention is that syscall 42 is to exit the program
    // In this case, a0 keeps the exit status
    // We use registers a0 to a5 as parameters for the syscall
    // we generalize it later for other kind of syscalls
    unsafe {
        asm!("li a0, 1", "li a7, 42", "ecall");
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
