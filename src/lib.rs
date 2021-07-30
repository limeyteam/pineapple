#![no_std]
#![feature(abi_x86_interrupt)]
pub mod drivers;
use x86_64;
use core::panic::PanicInfo;

#[macro_export]
macro_rules! println {
    () => (serial_print!("\n"));
    ($fmt:expr) => (serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC!, Did I do that?\n Panic info: {}", info);
    hlt_loop();
}
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
pub fn init() {
    drivers::interrupts::init_int();
}