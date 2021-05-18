#![no_std]
#![no_main]

// mod vga_buffer;

use core::panic::PanicInfo;
use os::{print, println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World.........{}", "!");

    os::init();
    // x86_64::instructions::interrupts::int3();
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    println!("It did not crash!");
    os::hlt_loop();
}
