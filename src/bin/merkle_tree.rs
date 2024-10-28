#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub use o1vm_stdlib;

// Entry point of our program
#[no_mangle]
pub extern "C" fn _start() -> () {
    loop {
        let data = [1, 2, 3, 4, 5, 6, 7, 8];
        let merkle_tree = o1vm_stdlib::merkle_tree::build(data);
        assert_eq!(merkle_tree.root, 1);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
