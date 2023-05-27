use clap::Parser;

#[derive(Parser, Debug)]
pub struct CleanCommand {
    category: String,
    repository: String,
}

impl CleanCommand {
    pub fn handle(&self) {
        let user_config = grass::dev::config::load_user_config().unwrap_or_default();

        grass::dev::clean_repository(&user_config, &self.category, &self.repository);
    }
}
