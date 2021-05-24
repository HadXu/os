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

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number + 100);
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World.........{}", "!");
    os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // let mut executor = SimpleExecutor::new();
    // executor.spawn(Task::new(example_task()));
    // executor.run();

    let heap_value = Box::new(41);
    println!("heap_value is {} at {:p}", *heap_value, heap_value);

    // let mut vec = Vec::new();
    // for i in 0..500 {
    //     vec.push(i);
    // }
    // println!("vec at {:p}", vec.as_slice());

    // let reference_counted = Rc::new(vec![1, 2, 3]);
    // let cloned_reference = reference_counted.clone();

    // println!(
    //     "current reference count is {}",
    //     Rc::strong_count(&cloned_reference)
    // );

    // core::mem::drop(reference_counted);
    // println!(
    //     "reference count is {} now",
    //     Rc::strong_count(&cloned_reference)
    // );

    // use x86_64::structures::paging::Page;
    // use x86_64::{structures::paging::Translate, VirtAddr};

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let mut mapper = unsafe { memory::init(phys_mem_offset) };

    // let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // let page = Page::containing_address(VirtAddr::new(0xbeaf000));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let mapper = unsafe { memory::init(phys_mem_offset) };

    // let addresses = [
    //     // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_info.physical_memory_offset,
    // ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    // use os::memory::active_level_4_table;
    // use x86_64::structures::paging::PageTable;
    // use x86_64::VirtAddr;

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };
    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };
    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("  L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //         // println!("L4 Entry {}: {:?}", i, entry);
    //     }
    // }

    println!("It did not crash!");
    os::hlt_loop();
}

// #[no_mangle]
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
//     println!("Hello World.........{}", "!");

//     os::init();
//     // x86_64::instructions::interrupts::int3();
//     // unsafe {
//     //     *(0xdeadbeef as *mut u64) = 42;
//     // };

//     // fn stack_overflow() {
//     //     stack_overflow();
//     // }

//     // stack_overflow();

//     // let ptr = 0x2031b2 as *mut u32;
//     // unsafe { let x = *ptr; }
//     // println!("read worked");

//     // unsafe { *ptr = 42; }
//     // println!("write worked");

//     use x86_64::registers::control::Cr3;
//     let (level_4_page_table, _) = Cr3::read();
//     println!(
//         "Level 4 page table at: {:?}",
//         level_4_page_table.start_address()
//     );

//     println!("It did not crash!");
//     os::hlt_loop();
// }
