mod cli;

use clap::Parser;
use cli::{execute_args, CliArgs};

fn main() {
    let args: CliArgs = CliArgs::parse();

    execute_args(&args)
}
