CURR_DIR = $(shell pwd)
DEPS_DIR = $(CURR_DIR)/_o1vm_deps
BASE_DIR_RISCV_TOOLS = $(DEPS_DIR)/riscv-gnu-toolchain
BASE_DIR_RISCV_TOOLS_BUILD = $(DEPS_DIR)/riscv-gnu-toolchain/build
OBJDUMP_BINARY = $(BASE_DIR_RISCV_TOOLS_BUILD)/bin/riscv32-unknown-elf-objdump

TARGET = "riscv32i-unknown-none-elf"

setup-toolchain:
	rustup target add $(TARGET)

setup-tool:
	mkdir -p _o1vm_deps
	if [ ! -d $(BASE_DIR_RISCV_TOOLS) ]; then git clone https://github.com/riscv-collab/riscv-gnu-toolchain $(BASE_DIR_RISCV_TOOLS) ; fi
	cd $(BASE_DIR_RISCV_TOOLS) && ./configure --with-arch=rv32gc --with-abi=ilp32d --prefix=$(BASE_DIR_RISCV_TOOLS_BUILD)
	cd $(BASE_DIR_RISCV_TOOLS) && make -j 32

setup: setup-toolchain setup-tool

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

print-executable-code:
	if [ -z $(O1VM_EXECUTABLE_NAME) ]; then echo "Please set the variable O1VM_EXECUTABLE_NAME to the program to analyze"; exit 1; fi
	if [ ! -f target/$(TARGET)/release/$(O1VM_EXECUTABLE_NAME) ]; then echo "Executable not found"; exit 1; fi
	$(OBJDUMP_BINARY) -d target/$(TARGET)/release/$(O1VM_EXECUTABLE_NAME) -d

.PHONY: setup-toolchain setup-tool setup build run clean test-doc fmt lint print-executable-code
