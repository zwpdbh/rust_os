# get rid of metadata to manually help Qemu to load the executable
REMOVE_META = rust-objcopy --strip-all ../target/riscv64gc-unknown-none-elf/release/ch01 -O binary ../target/riscv64gc-unknown-none-elf/release/ch01

run:
	cargo build --release && \
	$(REMOVE_META) && \
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=../target/riscv64gc-unknown-none-elf/release/ch01,addr=0x80200000