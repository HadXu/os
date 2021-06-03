use crate::{kernel, print, println, user};
use alloc::string::String;
use alloc::vec::Vec;

pub struct Shell {
    cmd: String,
    prompt: String,
    history: Vec<String>,
    history_index: usize,
}

impl Shell {
    pub fn new() -> Self {
        Shell {
            cmd: String::new(),
            prompt: String::from("> "),
            history: Vec::new(),
            history_index: 0,
        }
    }

    pub fn run(&mut self) {
        print!("\n");
        print!("{}", self.prompt);
        loop {
            let c = kernel::console::get_char();
            match c {
                '\n' => {
                    print!("\n");
                    if self.cmd.len() > 0 {
                        let line = self.cmd.clone();
                        self.history.push(line.clone());
                        self.history_index = self.history.len();
                        self.exec(&line);
                        self.cmd.clear();
                    }
                    print!("{}", self.prompt);
                }
                '↑' => {
                    if self.history.len() > 0 {
                        if self.history_index > 0 {
                            self.history_index -= 1;
                        }
                        let cmd = &self.history[self.history_index];
                        print!("{}", cmd);
                    }
                },
                '↓' => {
                    print!("down");
                }, 
                c => {
                    if c.is_ascii() && kernel::vga_buffer::is_printable(c as u8) {
                        self.cmd.push(c);
                        print!("{}", c);
                    }
                }
            }
        }
    }

    pub fn exec(&self, cmd: &str) {
        let args = self.parse(cmd);

        match args[0] {
            "date" => {
                user::date::main();
            },
            "sleep" => {
                user::sleep::main(&args);
            },
            "uptime" => {
                user::uptime::main();
            },
            _ => {
                println!("{}", "Unknown");
            }
        }
    }

    fn parse<'a>(&self, cmd: &'a str) -> Vec<&'a str> {
        let args: Vec<&str> = cmd.split_whitespace().collect();
        args
    }
}

pub fn main() {
    let mut shell = Shell::new();
    shell.run();
}
