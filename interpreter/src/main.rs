mod scanner;
mod tree;
use crate::scanner::Scanner;
use std::io::stdin;
use std::{env};
use std::fs;
mod test;
mod parser;

struct Interpreter {
    scanner: Scanner
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter { scanner: Scanner::default() }
    }
}

impl Interpreter {
    pub fn enter_line_mode(&mut self) {
        loop {
            let mut line = String::new();
            stdin().read_line(&mut line).unwrap();
            self.run_line(line);
            self.print_tokens();
        }
    }
    pub fn run_file(&mut self, code: String) {
        for line in code.split('\n') {
            self.run_line(line.to_string());
        }
    }
    fn run_line(&mut self, line: String) {
        if let Err(error) = self.scanner.scan_line(line.to_string()) {
            panic!("error found in line: {}, char: {}\n{}", self.scanner.line_no, self.scanner.char_index, error);
        }
    }
    pub fn print_tokens(&self) {
        println!("scanner: {:?}", self.scanner);
    }
}

fn main() {
    println!("Interpreter here!");
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::default();
    if args.len() == 1 {
        interpreter.enter_line_mode();
    }
    else {
        let content = fs::read_to_string(args[1].clone()).unwrap();
        interpreter.run_file(content);
        interpreter.print_tokens();
    }
}
