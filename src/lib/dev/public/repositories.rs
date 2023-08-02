use std::{fs, path::PathBuf};

use itertools::Itertools;

use crate::dev::{
    config::{GrassConfig, RootConfig},
    repository::{get_repository_changes, RepositoryChangeStatus},
    types::{FilteredCategoryDescription, SimpleCategoryDescription, SimpleRepositoryDescription},
};

/// List repositories in a single category
///
/// # Examples
///
/// ```
/// # use grass;
/// #
/// # let user_config = grass::dev::config::load_example_config();
///
/// // TODO: support dependency injection so this can be tested
/// // let result_general = grass::list_repos_by_category(&user_config, "general").unwrap();
/// // let result_work = grass::list_repos_by_category(&user_config, "work").unwrap();
///
/// // TODO: support dependency injection so this can be tested
/// // assert_eq!(vec!["first", "second"], result_general.repositories);
/// // assert_eq!("general", result_general.category);
/// // assert_eq!("work", result_work.category);
/// ```
pub fn list_repos_by_category<T>(
    user_config: &RootConfig,
    category_name: T,
) -> Option<SimpleCategoryDescription>
where
    T: AsRef<str>,
{
    let category = user_config
        .grass
        .get_from_category_or_alias(&category_name)?;
    let root_folder = user_config.grass.base_dir.join(&category.name);

    let repositories = if let Ok(entries) = fs::read_dir(root_folder) {
        entries
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| match entry.metadata() {
                Ok(metadata) if metadata.is_dir() => Some(entry.file_name().into_string().ok()?),
                _ => None,
            })
            .sorted()
            .map(|repository| SimpleRepositoryDescription {
                category: category.name.clone(),
                repository,
            })
    } else {
        return None;
    };

    Some(SimpleCategoryDescription::new(
        user_config,
        &category.name,
        repositories,
    ))
}

#[deprecated]
pub fn get_repository<T, U>(
    user_config: &RootConfig,
    category: T,
    repository: U,
) -> Option<SimpleRepositoryDescription>
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    let category = user_config.grass.get_from_category_or_alias(&category)?;

    let result = user_config
        .grass
        .base_dir
        .join(&category.name)
        .join(repository.as_ref());

    let dir_metadata = fs::metadata(result).ok()?;

    if !dir_metadata.is_dir() {
        return None;
    };

    Some(SimpleRepositoryDescription {
        category: String::from(&category.name),
        repository: String::from(repository.as_ref()),
    })
}

// TODO: Return a result instead of an Option
#[deprecated(note = "I will replace this with what is now `dev::get_repository_path_next`")]
pub fn get_repository_path<T, U>(
    user_config: &GrassConfig,
    category: T,
    repository: U,
) -> Option<PathBuf>
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    let category = user_config.get_from_category_or_alias(&category)?;

    let result = user_config
        .base_dir
        .join(&category.name)
        .join(repository.as_ref());

    let dir_metadata = fs::metadata(&result).ok()?;

    if !dir_metadata.is_dir() {
        return None;
    };

    Some(result)
}

// TODO: Add error handling
pub fn list_repositories_with_change_status<T>(
    user_config: &RootConfig,
    category: T,
) -> Option<
    FilteredCategoryDescription<
        impl Iterator<Item = (SimpleRepositoryDescription, RepositoryChangeStatus)>,
        RepositoryChangeStatus,
    >,
>
where
    T: AsRef<str>,
{
    let category = list_repos_by_category(user_config, category)?;
    let category_path = category.category_directory.clone();
    Some(FilteredCategoryDescription::new(
        user_config,
        category.category.clone(),
        // TODO: Handle errors more gracefully
        category.into_iter().filter_map(move |repository| {
            match get_repository_changes(category_path.join(&repository.repository)) {
                Ok(changes) => Some((repository, changes)),
                Err(_) => None,
            }
        }),
    ))
}

pub fn list_all_repositories_with_change_status(
    user_config: &RootConfig,
) -> Vec<
    FilteredCategoryDescription<
        impl Iterator<Item = (SimpleRepositoryDescription, RepositoryChangeStatus)> + '_,
        RepositoryChangeStatus,
    >,
> {
    user_config
        .grass
        .category
        .iter()
        .filter_map(|(key, _)| list_repositories_with_change_status(user_config, key))
        .collect()
}
