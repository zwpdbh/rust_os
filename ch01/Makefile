RELEASE_FILE = ../target/riscv64gc-unknown-none-elf/release/ch01
EXE_FILE = ../target/riscv64gc-unknown-none-elf/debug/ch01
BIOS_FILE = "../bootloader/rustsbi-qemu.bin"
OS_FILE = $(RELEASE_FILE).bin
KERNEL_ENTRY_PA = 0x80200000


# -s Let Qemu listen TCP port
gdb:
	cargo build --release && \
	$(REMOVE_META) && \
	$(LOAD_BIN) -s -S

connect: 
	riscv64-unknown-elf-gdb \
    -ex 'file $(EXE_FILE).bin' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'

asm:
	cargo build && \
	rust-objdump -S $(EXE_FILE)

user_run:
	cargo build && \
	qemu-riscv64 $(EXE_FILE)

# Get rid of metadata to manually help Qemu to load the executable
REMOVE_META = rust-objcopy --strip-all $(EXE_FILE) -O binary $(OS_FILE)
LOAD_BIN = qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BIOS_FILE) \
		-device loader,file=$(OS_FILE),addr=$(KERNEL_ENTRY_PA)

sys_run:
	cargo build --release && \
	$(REMOVE_META) && \
	$(LOAD_BIN)
