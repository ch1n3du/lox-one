use clap::Parser;
use lox_one::cli::{run_file, run_repl, Args};

fn main() {
    let args: Args = Args::parse();

    match args {
        Args::Repl => run_repl(false),
        Args::Run { src_path } => run_file(&src_path),
    }
}
