# RELEASE_FILE = ../target/riscv64gc-unknown-none-elf/release/ch01
# EXE_FILE = ../target/riscv64gc-unknown-none-elf/debug/ch01
# BIOS_FILE = "../bootloader/rustsbi-qemu.bin"
# OS_FILE = $(RELEASE_FILE).bin
# KERNEL_ENTRY_PA = 0x80200000


# # -s Let Qemu listen TCP port
# gdb:
# 	cargo build --release && \
# 	$(REMOVE_META) && \
# 	$(LOAD_BIN) -s -S

# connect: 
# 	riscv64-unknown-elf-gdb \
#     -ex 'file $(EXE_FILE).bin' \
#     -ex 'set arch riscv:rv64' \
#     -ex 'target remote localhost:1234'

# asm:
# 	cargo build && \
# 	rust-objdump -S $(EXE_FILE)

# user_run:
# 	cargo build && \
# 	qemu-riscv64 $(EXE_FILE)

# # Get rid of metadata to manually help Qemu to load the executable
# REMOVE_META = rust-objcopy --strip-all $(EXE_FILE) -O binary $(OS_FILE)
# LOAD_BIN = qemu-system-riscv64 \
# 		-machine virt \
# 		-nographic \
# 		-bios $(BIOS_FILE) \
# 		-device loader,file=$(OS_FILE),addr=$(KERNEL_ENTRY_PA)

# sys_run:
# 	cargo build --release && \
# 	$(REMOVE_META) && \
# 	$(LOAD_BIN)


# Building
TARGET := riscv64gc-unknown-none-elf
MODE := release
KERNEL_ELF := ../target/$(TARGET)/$(MODE)/ch01
KERNEL_BIN := $(KERNEL_ELF).bin
DISASM_TMP := ../target/$(TARGET)/$(MODE)/asm

# Building mode argument
ifeq ($(MODE), release)
	MODE_ARG := --release
endif

# BOARD
BOARD := qemu
SBI ?= rustsbi
BOOTLOADER := ../bootloader/$(SBI)-$(BOARD).bin

# KERNEL ENTRY
KERNEL_ENTRY_PA := 0x80200000

# Binutils
OBJDUMP := rust-objdump --arch-name=riscv64
OBJCOPY := rust-objcopy --binary-architecture=riscv64

# Disassembly
DISASM ?= -x

build: env $(KERNEL_BIN)

env:
	(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add $(TARGET)
	cargo install cargo-binutils
	rustup component add rust-src
	rustup component add llvm-tools-preview

$(KERNEL_BIN): kernel
	@$(OBJCOPY) $(KERNEL_ELF) --strip-all -O binary $@

kernel:
	@echo Platform: $(BOARD)
	@cargo build $(MODE_ARG)

clean:
	@cargo clean

disasm: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) | less

disasm-vim: kernel
	@$(OBJDUMP) $(DISASM) $(KERNEL_ELF) > $(DISASM_TMP)
	@vim $(DISASM_TMP)
	@rm $(DISASM_TMP)

run: run-inner

run-inner: build
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER) \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

debug: build
	@tmux new-session -d \
		"qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S" && \
		tmux split-window -h "riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'" && \
		tmux -2 attach-session -d

gdbserver: build
	@qemu-system-riscv64 -machine virt -nographic -bios $(BOOTLOADER) -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA) -s -S

gdbclient:
	@riscv64-unknown-elf-gdb -ex 'file $(KERNEL_ELF)' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'

.PHONY: build env kernel clean disasm disasm-vim run-inner gdbserver gdbclient
