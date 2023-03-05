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
        } => String::from(
            grass::get_category_path(&user_config, category)
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default(),
        ),
        PathCommand {
            category: _category,
            repository: Some(_repository),
        } => String::new(),
    };

    print!("{}", path);
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
