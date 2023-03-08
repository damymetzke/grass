use clap::Parser;
use grass::config;

#[derive(Parser, Debug)]
pub struct PathCommand {
    category: String,
    repository: Option<String>,
}

impl PathCommand {
    pub fn handle(&self) {
        let user_config = config::load_user_config().unwrap_or_default();
        let path = match self {
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
}
