use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[macro_use]
use crate::console::pintln;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
