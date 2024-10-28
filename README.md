# Test of a no-std program written in Rust for o1vm (riscv32i edition)

TL;DR

```
make setup
make build # will build fibonacci example
O1VM_EXECUTABLE_NAME=fibonacci make print-executable-code
```

You also have a standard library that can be used to replace components that are
not available in no-std environment (like elliptic curves, fields, digital
signature schemes, hash functions, etc)

```rust
use o1vm_stdlib;

// TODO [...]
```

------------

Add the riscv32i target:
```
rustup target add riscv32i-unknown-none-elf
```

Compile into RISC-V 32i
```
cargo build --release --target riscv32i-unknown-none-elf
```

The binary will be found in `target/riscv32i-unknown-none-elf/release/test`.

This project would eventually become part of o1-labs organizationa as a library
to provide a macro to create o1vm compatible programs

no-std programs are Rust programs that do not use the standard librar `std`. It
means programs do not suppose there is a network layer, nor allocators.
In this setting, no syscalls can be performed.
It is left to the user to implement them.
It is exactly what we need with zkVM: we want to reimplement syscalls to talk
with the external world.

To run proof-systems in o1vm, we will need to make one version of proof-systems
without std. It would be the endgoal of this project.
By using a no-std version of the verifier in proof-systems, and a no-std light
client like [helios](https://github.com/a16z/helios), we can have a bridge from
Ethereum to Mina.
We will need to modify the Mina protocol to accept o1vm proofs.

## Disassembler for RISC-V

### RISC-V GNU toolchain

Source:
```
git clone https://github.com/riscv-collab/riscv-gnu-toolchain
```

Compile for 32bits RISCV using:
```shell
./configure --with-arch=rv32gc --with-abi=ilp32d --prefix=$HOME/.bin/riscv-gnu-toolchain
make -j 32 # require a good internet connection and some minutes
```

You will find the binaries in `$HOME/.bin/riscv-gnu-toolchain`.

Alternatively, use [radare2](https://www.radare.org/n/radare2.html)

## Analyze generated code

To see the (executable) assembly code, use:
```
$HOME/.bin/riscv-gnu-toolchain/bin/riscv32-unknown-elf-objdump target/riscv32i-unknown-none-elf/release/test -d
```
It is the code to be loaded in the memory of the o1vm.
The entry point is normally the section starting with `_start`.

## Rust profiles

See https://doc.rust-lang.org/cargo/reference/profiles.html for compiler optimisation.

See more about optimisations [here](https://github.com/johnthagen/min-sized-rust)

## Some reminders

- `readelf -s [binary]`: list all symbols of the binary
- `$HOME/.bin/riscv-gnu-toolchain/bin/riscv32-unknown-elf-objdump [file] -d`:
shows the executable sections
- `$HOME/.bin/riscv-gnu-toolchain/bin/riscv32-unknown-elf-objdump [file] -D`:
shows all the sections. These sections will be required to be loaded in the o1vm
memory.
- `$HOME/.bin/riscv-gnu-toolchain/bin/riscv32-unknown-elf-objdump [file] -f`:
shows header information about the ELF. It does also suppose containing the
entry point of the program, often called `_start`. The entry point can also be
changed.
