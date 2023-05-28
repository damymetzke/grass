use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use grass::dev::types::{SimpleCategoryDescription, SimpleRepositoryDescription};

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

pub fn select_category_and_repository(
    categories: &[SimpleCategoryDescription],
) -> Option<&SimpleRepositoryDescription> {
    let sizes = categories
        .iter()
        .map(|category| category.repositories.len());
    let options: Vec<_> = categories
        .iter()
        .flat_map(|category| {
            category
                .repositories
                .iter()
                .map(|repository| format!("{}/{}", repository.category, repository.repository))
        })
        .collect();

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&options)
        .default(0)
        .vim_mode(true)
        .interact_opt()
        .unwrap_or(None)?;

    let mut j = selection;
    let mut i = 0;

    for size in sizes {
        if j < size {
            break;
        }
        j -= size;
        i += 1;
    }

    categories
        .get(i)
        .and_then(|category| category.repositories.get(j))
}
