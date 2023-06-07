use clap::Parser;
use grass::dev::config;

#[derive(Parser, Debug)]
/// Print the path of a category root, or a repository
///
/// The output will be a single string with no trailing newline.
pub struct PathCommand {
    /// The category to display, can be an alias
    category: String,
    /// The repository to display
    ///
    /// When omitted will print the repository directory instead.
    repository: Option<String>,
}

impl PathCommand {
    pub fn handle(&self) {
        // TODO: Handle errors
        let user_config = config::load_user_config().unwrap();
        let path = match self {
            PathCommand {
                category,
                repository: None,
            } => grass::dev::get_category_path(&user_config.grass, category),
            PathCommand {
                category,
                repository: Some(repository),
            } => grass::dev::get_repository_path(&user_config.grass, category, repository),
        };

        print!("{}", path.unwrap_or_default().to_str().unwrap_or_default());
    }
}
