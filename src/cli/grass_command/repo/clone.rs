use clap::Parser;
use grass::dev::config;

use crate::facades::dialoguer::select_selectable;

#[derive(Parser, Debug)]
pub struct CloneCommand {
    remote: String,
    category: Option<String>,
}

impl CloneCommand {
    // TODO: Handle errors
    pub fn handle(&self) {
        let user_config = config::load_user_config().unwrap_or_default();
        let category = self.category.clone().or_else(|| {
            select_selectable(&grass::dev::list_categories(&user_config)).cloned()
        }).unwrap();

        grass::dev::clone_repository(&user_config, &self.remote, category);
    }
}
