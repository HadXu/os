use crate::{kernel, print};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;

lazy_static! {
    pub static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(
            layouts::Us104Key,
            ScancodeSet1,
            HandleControl::MapLettersToUnicode
        ));
}

fn read_scancode() -> u8 {
    let mut port = Port::new(0x60);
    unsafe { port.read() }
}

pub fn interrupt_handler() {
    let mut keyboard = KEYBOARD.lock();
    let scancode = read_scancode();

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            let c = match key {
                DecodedKey::Unicode(c) => c,
                DecodedKey::RawKey(KeyCode::ArrowLeft) => '←', // U+2190
                DecodedKey::RawKey(KeyCode::ArrowUp) => '↑',   // U+2191
                DecodedKey::RawKey(KeyCode::ArrowRight) => '→', // U+2192
                DecodedKey::RawKey(KeyCode::ArrowDown) => '↓', // U+2193
                DecodedKey::RawKey(_) => {
                    return;
                }
            };
            kernel::console::key_handle(c);
            // print!("{}", c);
        }
    }
}

pub fn init() {
    kernel::interrupts::set_irq_handler(1, interrupt_handler);
}
