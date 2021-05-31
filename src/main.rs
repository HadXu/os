#![no_std]
#![no_main]

// mod vga_buffer;

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

use os::{kernel, println};

entry_point!(kernel_main);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World.........{}", "!");
    os::init(boot_info);

    let mut t = kernel::cmos::CMOS::new();
    let time = t.rtc();
    println!(
        "{} {} {} {} {}",
        time.year, time.month, time.day, time.hour, time.minute
    );

    loop {}
}
