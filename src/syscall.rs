use core::arch::asm;

pub const SYS_EXIT: i32 = 42;
pub const SYS_MMAP: i32 = 379;
pub const SYS_EXIT_SUCCESS: i32 = 0;
pub const SYS_EXIT_PANIC: i32 = 1;

/// Exit the program with success.
/// Under the hood, it generates assembly instruction with the following convention:
/// - The error code is set to [SYS_EXIT_SUCCESS] in the first register, a0. -
/// The syscall number for exit is [SYS_EXIT].
/// - The ecall instruction is used to trigger the syscall.
/// - The other registers used for the function calls (a1-a6) are set to zeros
///   for completeness.
pub fn exit_success() {
    unsafe {
        asm!(
            "li a0, 0",
            "li a1, 0",
            "li a2, 0",
            "li a3, 0",
            "li a4, 0",
            "li a5, 0",
            "li a6, 0",
            "li a7, 42",
            "ecall",
        );
    }
}

/// Exit the program with the panic error code 1.
/// Under the hood, it generates assembly instruction with the following convention:
/// - The error code is set to [SYS_EXIT_PANIC] in the first register, a0. - The
/// syscall number for exit is [SYS_EXIT].
/// - The ecall instruction is used to trigger the syscall.
/// - The other registers used for the function calls (a1-a6) are set to zeros
pub fn exit_panic() {
    unsafe {
        asm!(
            "li a0, 1",
            "li a1, 0",
            "li a2, 0",
            "li a3, 0",
            "li a4, 0",
            "li a5, 0",
            "li a6, 0",
            "li a7, 42",
            "ecall",
        );
    }
}

pub fn mmap(addr: usize, len: usize) {
    unsafe {
        asm!(
            "li a0, {addr}",
            "li a1, {len}",
            "li a2, 0",
            "li a3, 0",
            "li a4, 0",
            "li a5, 0",
            "li a6, 0",
            "li a7, 379",
            "ecall",
            addr = const addr,
            len = const len,
        );
    }
}
