use clap::{Parser, Subcommand};
use grass::list_categories;
use itertools::Itertools;

#[derive(Debug, Subcommand)]
enum Command {
    Ls,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Ls => println!(
            "Categories:\n\n{}",
            list_categories()
                .iter()
                .map(|category| format!("* {}", category))
                .join("\n")
        ),
    }
}
