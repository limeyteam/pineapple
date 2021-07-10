#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#[cfg(test)]
use bootloader::{entry_point, BootInfo};


pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;

use core::panic::PanicInfo;
use ansi_rgb::{ Foreground, red, green};

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
    let version = "0.1.1";
    println!("Spinix Kernel, Version: {0} \n", version);
    println!("GDT [OK]");
    println!("Interrupts [OK]");
    println!("PIC [OK]");
    println!("PIC [Enabled]");
    println!("_.");
    println!("VGA Buffer [Tested]");
    println!("Shell Starting...");
    shellinit();
}

pub fn collectionTask() {
    let test = interrupts::collect();
    println!("{}", test);
}

static HELLO: &[u8] = b"Hello World!";

pub fn hello() {
    println!("Hello World{} \n", "!");
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();         // new
}

#[cfg(test)]
entry_point!(test_kernel_main);
/// Entry point for `cargo test`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    // like before
    init();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn shellinit() {
    println!(">");
}
pub fn eshell(lestring: &str) {
    if lestring == "ping" {
        println!("Pong!");
    }
    shellinit();
}
pub fn charprint(lestring: &str) {
    print!("{}", lestring);
}