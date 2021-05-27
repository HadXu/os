use crate::{gdt, print, println};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use pic8259::ChainedPics;
use spin;

use crate::hlt_loop;
use x86_64::structures::idt::PageFaultErrorCode;

fn interrupt_index(irq: u8) -> u8 {
    PIC_1_OFFSET + irq
}

fn default_handler() {
    return;
}

macro_rules! irq_handler {
    ($handler:ident, $irq:expr) => {
        pub extern "x86-interrupt" fn $handler(_stack_frame: InterruptStackFrame) {
            let handlers = IRQ_HANDLERS.lock();
            handlers[$irq]();
            unsafe { PICS.lock().notify_end_of_interrupt(interrupt_index($irq)); }
        }
    };
}

irq_handler!(irq0_handler, 0);
irq_handler!(irq1_handler, 1);
irq_handler!(irq2_handler, 2);
irq_handler!(irq3_handler, 3);
irq_handler!(irq4_handler, 4);
irq_handler!(irq5_handler, 5);
irq_handler!(irq6_handler, 6);
irq_handler!(irq7_handler, 7);
irq_handler!(irq8_handler, 8);
irq_handler!(irq9_handler, 9);
irq_handler!(irq10_handler, 10);
irq_handler!(irq11_handler, 11);
irq_handler!(irq12_handler, 12);
irq_handler!(irq13_handler, 13);
irq_handler!(irq14_handler, 14);
irq_handler!(irq15_handler, 15);


lazy_static! {
    pub static ref IRQ_HANDLERS: Mutex<[fn(); 16]> = Mutex::new([default_handler; 16]);
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[interrupt_index(0) as usize].set_handler_fn(irq0_handler);
        idt[interrupt_index(1) as usize].set_handler_fn(irq1_handler);
        idt[interrupt_index(2) as usize].set_handler_fn(irq2_handler);
        idt[interrupt_index(3) as usize].set_handler_fn(irq3_handler);
        idt[interrupt_index(4) as usize].set_handler_fn(irq4_handler);
        idt[interrupt_index(5) as usize].set_handler_fn(irq5_handler);
        idt[interrupt_index(6) as usize].set_handler_fn(irq6_handler);
        idt[interrupt_index(7) as usize].set_handler_fn(irq7_handler);
        idt[interrupt_index(8) as usize].set_handler_fn(irq8_handler);
        idt[interrupt_index(9) as usize].set_handler_fn(irq9_handler);
        idt[interrupt_index(10) as usize].set_handler_fn(irq10_handler);
        idt[interrupt_index(11) as usize].set_handler_fn(irq11_handler);
        idt[interrupt_index(12) as usize].set_handler_fn(irq12_handler);
        idt[interrupt_index(13) as usize].set_handler_fn(irq13_handler);
        idt[interrupt_index(14) as usize].set_handler_fn(irq14_handler);
        idt[interrupt_index(15) as usize].set_handler_fn(irq15_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    panic!();
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

// extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
//     print!(".");
//     unsafe {
//         PICS.lock()
//             .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
//     }
// }

// extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
//     use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
//     use spin::Mutex;
//     use x86_64::instructions::port::Port;

//     lazy_static! {
//         static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
//             Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
//         );
//     }
//     let mut keyboard = KEYBOARD.lock();
//     let mut port = Port::new(0x60);
//     let scancode: u8 = unsafe { port.read() };

//     if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
//         if let Some(key) = keyboard.process_keyevent(key_event) {
//             match key {
//                 DecodedKey::Unicode(character) => print!("{}", character),
//                 DecodedKey::RawKey(key) => print!("{:?}", key),
//             }
//         }
//     }

//     unsafe {
//         PICS.lock()
//             .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
//     }
// }

// extern "x86-interrupt" fn page_fault_handler(
//     stack_frame: InterruptStackFrame,
//     error_code: PageFaultErrorCode,
// ) {
//     use x86_64::registers::control::Cr2;

//     println!("EXCEPTION: PAGE FAULT");
//     println!("Accessed Address: {:?}", Cr2::read());
//     println!("Error Code: {:?}", error_code);
//     println!("{:#?}", stack_frame);
//     hlt_loop();
// }
