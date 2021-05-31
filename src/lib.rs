#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]

extern crate alloc;
use bootloader::BootInfo;

pub mod kernel;
pub mod user;

pub fn init(boot_info: &'static BootInfo) {
    kernel::gdt::init();
    kernel::interrupts::init_idt();
    unsafe { kernel::interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();

    kernel::cpu::init();
    kernel::memory::init(boot_info);
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    println!("allocation error: {:?}", layout);
    panic!("error")
}
