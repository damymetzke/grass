use clap::Parser;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use grass::dev::config;

#[derive(Parser, Debug)]
pub struct CloneCommand {
    remote: String,
    category: Option<String>,
}

impl CloneCommand {
    // TODO: Handle errors
    pub fn handle(&self) {
        let user_config = config::load_user_config().unwrap_or_default();
        let category = self.category.clone().unwrap_or_else(|| {
            let categories = grass::dev::list_categories(&user_config);
            let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                .items(&categories)
                .default(0)
                .vim_mode(true)
                .interact_opt()
                .unwrap()
                .unwrap();

            categories[selection].clone()
        });

        grass::dev::clone_repository(&user_config, &self.remote, category);
    }
}
