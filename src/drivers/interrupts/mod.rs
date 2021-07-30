#![no_std]
#[macro_use]
pub mod cpu;
pub mod gdt;
use crate::hlt_loop;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(cpu::breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(cpu::double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

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
}