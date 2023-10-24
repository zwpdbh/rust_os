# Chapter 01

## General

- Create `.cargo/config` in the project root to config how to build.
- Check the build file in the **root of workspace**
  
  - Check file format: `file target/riscv64gc-unknown-none-elf/debug/ch01_user_space`
  - Check file header info: `rust-readobj -h target/riscv64gc-unknown-none-elf/debug/ch01_user_space`
  - Analyze the assembly code: `rust-objdump -h target/riscv64gc-unknown-none-elf/debug/ch01_user_space`

- TODO:: [构建用户态执行环境](http://learningos.cn/rCore-Tutorial-Guide-2023A/chapter1/3mini-rt-usrland.html)

## Troubleshooting

- When `use core::panic::PanicInfo;` shows error could not find crate core.
  Solution: `rustup target add riscv64gc-unknown-none-elf`

- When `rust-readobj` shows error didn't find it.
  Solution: `rustup component add llvm-tools-preview`

- Stuck in QEMU terminal 
  Solution: Ctrl + a, x 

# References

- [rCore-Tutorial-Guide 2023 秋季学期](http://learningos.cn/rCore-Tutorial-Guide-2023A/index.html)
- [rCore-Tutorial-Book 第三版](https://rcore-os.cn/rCore-Tutorial-Book-v3/index.html)