#![no_std]

pub mod syscall;

pub const PAGE_SIZE: usize = 4096;

// unsafe fn alloc_page() -> *mut u8 {
//     let page = syscall::mmap(0, PAGE_SIZE, syscall::PROT_READ | syscall::PROT_WRITE, syscall::MAP_ANONYMOUS | syscall::MAP_PRIVATE, -1, 0);
//     if page == syscall::MAP_FAILED {
//         panic!("mmap failed");
//     }
//     page as *mut u8
// }
