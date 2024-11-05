use core::arch::asm;

/// Exit the program with the given error code.
/// Under the hood, it generates assembly instruction with the following convention:
/// - The error code is passed in the `a0` register.
/// - The syscall number for exit is 42.
/// - The ecall instruction is used to trigger the syscall.
/// - The other registers used for the function calls (a1-a6) are set to zeros
/// for completeness.
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
