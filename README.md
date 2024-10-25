# Test of a no-std program written in Rust for o1vm

Compile into RISC-V 32i
```
cargo build --release --target riscv32i-unknown-none-elf
```

This project would eventually become part of o1-labs organizationa as a library
to provide a macro to create o1vm compatible programs

## Disassembler for RISC-V

### RISC-V GNU toolchain

Source: https://github.com/riscv-collab/riscv-gnu-toolchain
Compile for 32bits RISCV using:
```shell
./configure --with-arch=rv32gc --with-abi=ilp32d --prefix=$HOME/.bin/riscv-gnu-toolchain
```

You will find the binaries in `$HOME/.bin/riscv-gnu-toolchain`.

Alternatively, use [radare2](https://www.radare.org/n/radare2.html)
