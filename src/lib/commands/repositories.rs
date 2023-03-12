use std::{fs, path::PathBuf};

use itertools::Itertools;

use crate::{
    config::RootConfig,
    types::{SimpleCategoryDescription, SimpleRepositoryDescription},
    util::get_base_directory,
};

/// List repositories in a single category
///
/// # Examples
///
/// ```
/// # use grass::list_repos_by_category;
///
/// let result_general = list_repos_by_category("general").unwrap();
/// let result_work = list_repos_by_category("work").unwrap();
///
/// assert_eq!(vec!["first", "second"], result_general.repositories);
/// assert_eq!("general", result_general.category);
/// assert_eq!("work", result_work.category);
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
    let root_folder = get_base_directory(user_config)?.join(&category.name);

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
        category_name.as_ref(),
        repositories,
    ))
}

pub fn list_all_repositories(user_config: &RootConfig) -> Vec<SimpleCategoryDescription> {
    user_config
        .grass
        .category
        .iter()
        .filter_map(|(key, _)| list_repos_by_category(user_config, key))
        .collect()
}

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

    let result = get_base_directory(user_config)?
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

pub fn get_repository_path<T, U>(
    user_config: &RootConfig,
    category: T,
    repository: U,
) -> Option<PathBuf>
where
    T: AsRef<str>,
    U: AsRef<str>,
{
    let category = user_config.grass.get_from_category_or_alias(&category)?;

    let result = get_base_directory(user_config)?
        .join(&category.name)
        .join(repository.as_ref());

    let dir_metadata = fs::metadata(&result).ok()?;

    if !dir_metadata.is_dir() {
        return None;
    };

    Some(result)
}
