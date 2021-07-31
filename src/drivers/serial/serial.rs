#![no_std]
use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

pub fn serial_init(serial) {
    i = 1;
lazy_static! {
    pub static ref SERIAL(i): Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3FF) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;       // new

    interrupts::without_interrupts(|| {         // new
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}