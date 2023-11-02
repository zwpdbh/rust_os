// Tell rust compiler that we are not going to use core instead of std
#![no_std]
#![no_main]

mod lang_items;
mod sbi;

#[macro_use]
mod console;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bass();
    println!("Hello, world!");
    panic!("Shutdown machine!");
}

fn clear_bass() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
