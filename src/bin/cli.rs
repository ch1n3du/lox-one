use std::io::{self, Write};

use clap::Parser;
use colored::Colorize;

use lox_one::{error::LoxError, interpreter::Interpreter};

#[derive(Parser)]
#[command(name = "lox_one")]
#[command(
    about = "A tree-walking Lox Interpreter written by Ch1n3du.",
    long_about = "This is small project I built to learn about compilers."
)]
pub enum CliArgs {
    #[command(about = "Runs the lox_one REPL.")]
    Repl,
    #[command(about = "Runs the given Lox program file.")]
    Run { src_path: String },
}

pub fn execute_args(args: &CliArgs) {
    use CliArgs::*;
    match args {
        Repl => run_repl(false),
        Run { src_path } => run_file(src_path),
    }
}

fn run_file(src_path: &str) {
    let src = std::fs::read_to_string(src_path).expect(&format!("Error finding file {src_path}"));
    Interpreter::new().interpret_str(&src).unwrap_or_else(|e| {
        println!("{e}");
        panic!()
    });
}

fn run_repl(_verbose: bool) {
    let mut interpreter = Interpreter::new();
    println!("Lox Interpreter Version 1.0.0");
    println!("Enter 'exit' or ':q' to quit.");

    'a: loop {
        print!("{} ", "λ> ".cyan().bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            println!("{}", LoxError::IO(e));
            break 'a;
        }

        if input.starts_with(":q") || input.starts_with("exit") {
            println!(
                "{}",
                "Goodbye and thanks for all the fish ><> ><>".green().bold(),
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

fn main() {}
