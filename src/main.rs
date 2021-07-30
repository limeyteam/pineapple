#![no_std]
#![no_main]
#[macro_use]
use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use spinix::hlt_loop;
use spinix::drivers::*;
use spinix;
use spinix::serial_println;
use spinix::init;

entry_point!(kernel_main);


fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init();
    serial_println!("Bootloader version: {0}.{1}.{2}", boot_info.version_major, boot_info.version_minor, boot_info.version_patch);
    x86_64::instructions::interrupts::int3();
    panic!("test");
    hlt_loop();
    loop{}
}