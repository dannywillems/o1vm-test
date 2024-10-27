TARGET = "riscv32i-unknown-none-elf"

setup:
	rustup target add $(TARGET)

# Build the project
build:
	cargo build --release --target $(TARGET)

# Run the project
# FIXME: No. Add instructions to install qemu
run:
	qemu-system-riscv32 -machine sifive_e -nographic -bios none -kernel target/$(TARGET)/release/test

# Clean the project
clean:
	cargo clean

# Test the project's docs comments
test-doc:
	cargo test --all-features --release --doc

fmt:
	cargo +nightly fmt -- --check

# Lint the code
lint:
	cargo clippy --all-features --all-targets -- -W clippy::all -D warnings
