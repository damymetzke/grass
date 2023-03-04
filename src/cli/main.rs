mod more_itertools;

use clap::{Parser, Subcommand};
use grass::{config, list_categories, list_repos_by_category};
use itertools::Itertools;
use more_itertools::MoreItertools;

#[derive(Parser, Debug)]
struct LsCommand {
    category: Option<String>,
    #[clap(short, long)]
    all: bool,
}

fn handle_ls(command: LsCommand) {
    let user_config = config::load_user_config().unwrap_or_default();
    let category = if let Some(result) = command.category {
        result
    } else if command.all {
        let categories = grass::list_all_repositories(&user_config);

        println!(
            "{}",
            categories
                .iter()
                .map(|category| format!(
                    "┌ Repos for category '{}':\n│\n{}",
                    category.category,
                    category
                        .repositories
                        .iter()
                        .sandwich_map(
                            |category_name| format!("├─ {}", category_name),
                            |category_name| format!("├─ {}", category_name),
                            |category_name| format!("└─ {}", category_name),
                        )
                        .join("\n")
                ))
                .join("\n\n")
        );
        return;
    } else {
        let categories = list_categories(&user_config);
        println!(
            "┌ Categories:\n│\n{}",
            categories
                .iter()
                .sandwich_map(
                    |category_name| format!("├─ {}", category_name),
                    |category_name| format!("├─ {}", category_name),
                    |category_name| format!("└─ {}", category_name),
                )
                .join("\n")
        );
        return;
    };

    let (category, repositories) =
        if let Some(category) = list_repos_by_category(&user_config, &category) {
            (category.category, category.repositories)
        } else {
            eprintln!("Category '{}' does not exist", &category);
            return;
        };
    println!(
        "┌ Repos for category '{}':\n│\n{}",
        category,
        repositories
            .iter()
            .sandwich_map(
                |category| format!("├─ {}", category),
                |category| format!("├─ {}", category),
                |category| format!("└─ {}", category),
            )
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
