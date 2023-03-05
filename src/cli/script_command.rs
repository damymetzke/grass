use clap::{Parser, Subcommand};
use grass::config;

#[derive(Parser, Debug)]
struct PathCommand {
    category: String,
    repository: Option<String>,
}

fn handle_path(command: &PathCommand) {
    let user_config = config::load_user_config().unwrap_or_default();
    let path = match command {
        PathCommand {
            category,
            repository: None,
        } => grass::get_category_path(&user_config, category),
        PathCommand {
            category,
            repository: Some(repository),
        } => grass::get_repository_path(&user_config, category, repository),
    };

    print!("{}", path.unwrap_or_default().to_str().unwrap_or_default());
}

#[derive(Debug, Subcommand)]
enum Command {
    Path(PathCommand),
}

#[derive(Parser, Debug)]
pub struct ScriptCommand {
    #[command(subcommand)]
    command: Command,
}

pub fn handle_script(command: &ScriptCommand) {
    match &command.command {
        Command::Path(path_command) => handle_path(path_command),
    }
}
