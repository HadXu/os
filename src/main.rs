#![no_std]
#![no_main]

// mod vga_buffer;

use core::panic::PanicInfo;
use os::println;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World.........{}", "!");

    os::init();
    x86_64::instructions::interrupts::int3();

    println!("It did not crash!");

    loop {}
}
