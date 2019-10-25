use std;
use std::io;
use std::io::Write;

use crate::vm::VM;

use colored::*;

pub struct REPL {
    command_history: Vec<String>,
    vm: VM,
}

impl REPL {
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_history: vec![],
        }
    }

    fn exit(&mut self) {
        println!("");
        std::process::exit(0);
    }

    pub fn run(&mut self) {
        println!("{}{}", "ASMVM".yellow().bold(), "DB".red().italic());
        println!("v0.1\n");

        loop {
            let mut buffer = String::new();
            let stdin = io::stdin();

            print!("{} ", "->".bright_red());
            io::stdout().flush().expect("Unable to flush stdout");

            if stdin.read_line(&mut buffer).expect("Unable to read input") == 0 {
                // Handle EOF
                self.exit();
            }

            let buffer = buffer.trim();
            self.command_history.push(buffer.to_string());

            match buffer {
                ".quit" => {
                    self.exit();
                }
                ".history" => {
                    for command in &self.command_history {
                        println!("{}", command);
                    }
                }
                _ => {
                    println!("Invalid input: {:?}", buffer);
                }
            }
        }
    }
}
