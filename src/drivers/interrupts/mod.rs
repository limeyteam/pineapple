#![no_std]
#[macro_use]
pub mod cpu;
pub mod gdt;
use crate::hlt_loop;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use pic8259::ChainedPics;
use crate::println;

static mut tick: u128 = 0;


pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard, // new
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(cpu::breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(cpu::double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(timer_interrupt_handler); // new

        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard_interrupt_handler);

        idt.page_fault.set_handler_fn(cpu::page_fault_handler); // new

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

pub fn init_int() {
    init_idt();
    gdt::init_gdt();
    unsafe { PICS.lock().initialize() };
}

// PIC stuff because it doesn't work with other modules :(
    pub extern "x86-interrupt" fn timer_interrupt_handler(
        _stack_frame: InterruptStackFrame)
    {
        unsafe {
            tick += 1;
            PICS.lock()
                .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
        }
    }
    
    pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
        use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
        use spin::Mutex;
        use x86_64::instructions::port::Port;
    
        lazy_static! {
            static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
                Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
            );
        }
    
        let mut keyboard = KEYBOARD.lock();
        let mut port = Port::new(0x60);
    
        let scancode: u8 = unsafe { port.read() };
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => println!("{}", character),
                    DecodedKey::RawKey(key) => println!("{:?}", key),
                }
            }
        }
    
        unsafe {
            PICS.lock()
                .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
        }
    }