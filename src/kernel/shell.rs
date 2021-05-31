use crate::print;
use alloc::string::String;

use x86_64::instructions::interrupts;

pub struct Shell {
    cmd: String,
    prompt: String,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            cmd: String::new(),
            prompt: String::from("> "),
        }
    }
    pub fn run(&mut self) {
        print!("\n");
        print!("{}", self.prompt);
        loop {

        }
    }

}


pub fn main() {
    let mut shell = Shell::new();
    shell.run();
}