# Chapter 01

## General

- Create `.cargo/config` in the project root to config how to build.
- Check the build file in the **root of workspace**

  - Check file format: `file target/riscv64gc-unknown-none-elf/debug/ch01_user_space`
  - Check file header info: `rust-readobj -h target/riscv64gc-unknown-none-elf/debug/ch01_user_space`
  - Analyze the assembly code: `rust-objdump -h target/riscv64gc-unknown-none-elf/debug/ch01_user_space`

- What is the metadata in the executable file generated by `cargo build --release`?
- How to get rid of those metadata? Such that the RustSBI could find the first instruction at `0x80200000`

- We need to write some assembly code in `entry.asm` to handle the control to RustSBI.

- [Registers related with function call](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter1/5support-func-call.html) 
  - ra 
  - fp 
  - sp 

- Allocate and use boost stack 
  - Stack grow from high address to low address.
  - From high to low, use 3 variables: bottom -> top -> lower_bound. 
    - When top == bottom, there is no stack allocation.
    - When top == lower_bound, stack is full.
  

## Troubleshooting

- When `use core::panic::PanicInfo;` shows error could not find crate core.
  Solution: `rustup target add riscv64gc-unknown-none-elf`

- When `rust-readobj` shows error didn't find it.
  Solution: `rustup component add llvm-tools-preview`

- Stuck in QEMU terminal
  Solution: Ctrl + a, x

- "riscv64-unknown-elf-gdb: Command not found"
  Solution:

  ```sh
  cd ~/
  wget https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.3.0-2020.04.1-x86_64-linux-ubuntu14.tar.gz
  tar zxvf riscv64-unknown-elf-gcc-8.3.0-2020.04.1-x86_64-linux-ubuntu14.tar.gz

  export PATH=$PATH:/home/zw/riscv64-unknown-elf-gcc-8.3.0-2020.04.1-x86_64-linux-ubuntu14/bin
  riscv64-unknown-elf-gdb -v
  ```

  - (Optional) install GDB dashboard:
    - Just place .gdbinit in your home directory, for example with: `wget -P ~ https://git.io/.gdbinit`
    - Then debug as usual, the dashboard will appear automatically every time the inferior program stops.

# References

- [rCore-Tutorial-Guide 2023 秋季学期](http://learningos.cn/rCore-Tutorial-Guide-2023A/index.html)
- [rCore-Tutorial-Book 第三版](https://rcore-os.cn/rCore-Tutorial-Book-v3/index.html)
