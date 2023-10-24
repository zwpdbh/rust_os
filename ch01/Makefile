run:
	cargo build --release && \
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ../bootloader/rustsbi-qemu.bin \
		-device loader,file=../target/riscv64gc-unknown-none-elf/release/ch01,addr=0x80200000