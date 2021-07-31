#![no_std]
#![no_main]
#[macro_use]
#[feature(asm)]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use spinix::hlt_loop;
use spinix::drivers::*;
use spinix;
use spinix::println;
use spinix::init;
use bootloader::boot_info::{FrameBuffer, FrameBufferInfo};
use spinix::drivers::allocator::memory::active_level_4_table;
use x86_64::VirtAddr;
use spinix::drivers::allocator::memory::BootInfoFrameAllocator;
use alloc::alloc::Layout;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    // Initializes the kernel, Interrupts, Serial, Graphics, etc...
    init();
    spinix::drivers::gpu::gop::init_gop(boot_info.framebuffer.as_mut().unwrap());
    println!("{:#?}", boot_info);
    init_memory(boot_info);
    
    // Debug info for nerds
    

    hlt_loop();
    loop{}
}

fn init_memory(boot_info: &'static BootInfo) {
    use crate::BootInfoFrameAllocator;
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mut mapper = unsafe { spinix::drivers::allocator::memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_regions) };
    spinix::drivers::allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}