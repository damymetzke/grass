mod command;
#[cfg(debug_assertions)]
mod debug;
mod more_itertools;
mod script_command;

use clap::Parser;
use command::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let args = Args::parse();

    args.command.handle();
}
