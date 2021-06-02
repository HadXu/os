use crate::{kernel, print};
use alloc::string::String;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

lazy_static! {
    pub static ref STDIN: Mutex<String> = Mutex::new(String::new());
    pub static ref ECHO: Mutex<bool> = Mutex::new(true);
    pub static ref RAW: Mutex<bool> = Mutex::new(false);
}

pub fn disable_echo() {
    let mut echo = ECHO.lock();
    *echo = false;
}

pub fn enable_echo() {
    let mut echo = ECHO.lock();
    *echo = true;
}

pub fn is_echo_enabled() -> bool {
    *ECHO.lock()
}

pub fn disable_raw() {
    let mut raw = RAW.lock();
    *raw = false;
}

pub fn enable_raw() {
    let mut raw = RAW.lock();
    *raw = true;
}

pub fn is_raw_enabled() -> bool {
    *RAW.lock()
}

pub fn key_handle(key: char) {
    let mut stdin = STDIN.lock();
    if key == '\x08' && !is_raw_enabled() {
        if let Some(c) = stdin.pop() {
            if is_echo_enabled() {
                let n = match c {
                    '\x03' | '\x04' => 2,
                    _ => c.len_utf8(),
                };
                print!("{}", "\x08".repeat(n));
            }
        }
    } else {
        stdin.push(key);
        if is_echo_enabled() {
            match key {
                '\x03' => print!("^C"),
                '\x04' => print!("^D"),
                _ => print!("{}", key),
            };
        }
    }
}

pub fn get_char() -> char {
    kernel::console::disable_echo();
    kernel::console::enable_raw();
    loop {
        kernel::time::halt();
        let res = interrupts::without_interrupts(|| {
            let mut stdin = STDIN.lock();
            match stdin.chars().next_back() {
                Some(c) => {
                    stdin.clear();
                    Some(c)
                },
                _ => {
                    None
                }
            }
        });

        if let Some(c) = res {
            kernel::console::enable_echo();
            kernel::console::disable_raw();
            return c;
        }
    }
}
