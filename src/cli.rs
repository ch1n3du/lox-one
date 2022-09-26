use std::io::{self, Write};

use clap::Parser;
use colored::Colorize;

use crate::{error::LoxError, interpreter::Interpreter};

#[derive(Parser)]
pub enum Args {
    Repl,
    Run { src_path: String },
}

pub fn run_file(src_path: &str) {
    let src = std::fs::read_to_string(src_path).expect(&format!("Error finding file {src_path}"));
    Interpreter::new().interpret_str(&src).unwrap_or_else(|e| {
        println!("{e}");
        panic!()
    });
}

pub fn run_repl(_verbose: bool) {
    let mut interpreter = Interpreter::new();
    println!("Lox Interpreter Version 1.0.0");
    println!("Enter 'exit' or ':q' to quit.");

    'a: loop {
        print!("λ> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            println!("{}", LoxError::IO(e));
            break 'a;
        }

        if input.starts_with(":q") || input.starts_with("exit") {
            println!(
                "{}",
                "Goodbye and thanks for all the fish ><>".green().bold()
            );
            break 'a;
        }

        match interpreter.interpret_str(&input) {
            Ok(Some(v)) => {
                println!("{} {}", "Result:".blink().bold(), v);
            }
            Ok(_) => continue,
            Err(e) => {
                println!("{}", e);
            }
        };
    }
}
