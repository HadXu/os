#![no_std]
#![no_main]

// mod vga_buffer;

extern crate alloc;

use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use os::memory::translate_addr;
use os::memory::BootInfoFrameAllocator;
use os::task::{simple_executor::SimpleExecutor, Task};
use os::{allocator, memory, print, println};

use x86_64::{structures::paging::Translate, VirtAddr};

entry_point!(kernel_main);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World.........{}", "!");
    os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    loop {

    }
}
