use clap::{Parser, Subcommand};
use grass::{list_categories, list_repos_by_category};
use itertools::Itertools;

#[derive(Parser, Debug)]
struct LsCommand {
    category: Option<String>,
}

fn handle_ls(command: LsCommand) {
    let category = if let Some(result) = command.category {
        result
    } else {
        println!(
            "Categories:\n\n{}",
            list_categories()
                .iter()
                .map(|category| format!("* {}", category))
                .join("\n")
        );
        return;
    };

    let category = list_repos_by_category(category);
    println!(
        "Repos for category '{}':\n\n{}",
        category.category,
        category
            .repositories
            .iter()
            .map(|category| format!("* {}", category))
            .join("\n")
    );
}

#[derive(Debug, Subcommand)]
enum Command {
    Ls(LsCommand),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Ls(command) => handle_ls(command),
    }
}
