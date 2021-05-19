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

    // let ptr = 0x2031b2 as *mut u32;
    // unsafe { let x = *ptr; }
    // println!("read worked");

    // unsafe { *ptr = 42; }
    // println!("write worked");

    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!(
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    );

    println!("It did not crash!");
    os::hlt_loop();
}
