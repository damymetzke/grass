use clap::{Parser, Subcommand};
use grass::{list_categories, list_repos_by_category, config};
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
        println!("{:?}", config::load_user_config().unwrap());
        return;
    };

    let (category, repositories) = if let Some(category) = list_repos_by_category(&category) {
        (category.category, category.repositories)
    } else {
        eprintln!("Category '{}' does not exist", &category);
        return;
    };
    println!(
        "Repos for category '{}':\n\n{}",
        category,
        repositories
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
