## How to write Rust programs compatible with the o1vm?

The o1vm requires some changes of the Rust code because our zkvm supposes some
entry points and some memory layouts.

First, the entry point can be defined by using the macro `#[o1vm_main]` for the
main function of the program. For instance:

```rust
const MAX_ITERATIONS: u32 = 10000;

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

#[o1vm_main]
fn main() {
    let mut x = 1;
    loop {
        x += 1;
        let _ = fibonacci(x);
        if x == MAX_ITERATIONS {
            break;
        }
    }
}
```

From there, you can simply call `cargo build --release --target
riscv32i-unknown-none-elf` and load the binary in the o1vm.

What the code above does is replacing `main` with `_start`, which is the
expected symbol for the entry point in the o1vm.
The Rust macro `#[no_mangle]` is simply use in the VM for debugging. Without the
macro, the compiler replaces the function name. Using no_mangle will allow you
to see the correct symbol while running the o1vm, and debug it in an easier way.
It is a no-cost operation, you can remove it or add it as you prefer. Note that
the symbols have to be unique.
