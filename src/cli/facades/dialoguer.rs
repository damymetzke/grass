use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use grass::dev::types::SimpleRepositoryDescription;

pub fn select_repository(
    repositories: &[SimpleRepositoryDescription],
) -> Option<&SimpleRepositoryDescription> {
    let options: Vec<_> = repositories
        .iter()
        .map(|repository| &repository.repository)
        .collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&options)
        .default(0)
        .vim_mode(true)
        .interact_opt()
        .unwrap_or(None)?;

    repositories.get(selection)
}
