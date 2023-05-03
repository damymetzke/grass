use std::{fs, path::PathBuf};

use itertools::Itertools;

use crate::{
    config::RootConfig,
    repository::{get_repository_changes, RepositoryChangeStatus},
    types::{SimpleCategoryDescription, SimpleRepositoryDescription},
    util::get_base_directory,
};

/// List repositories in a single category
///
/// # Examples
///
/// ```
/// # use grass;
/// #
/// # let user_config = grass::config::load_example_config();
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
        &category.name,
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

pub fn list_repositories_with_change_status<T>(
    user_config: &RootConfig,
    category: T,
) -> Option<impl Iterator<Item = (SimpleRepositoryDescription, RepositoryChangeStatus)>>
where
    T: AsRef<str>,
{
    let category = list_repos_by_category(user_config, category)?;
    let category_path = category.category_directory.clone();
    Some(category.into_iter().map(move |repository| {
        (
            repository.clone(),
            get_repository_changes(category_path.join(&repository.repository)),
        )
    }))
}

pub fn get_uncommitted_repositories<T>(
    user_config: &RootConfig,
    category: T,
) -> Option<SimpleCategoryDescription>
where
    T: AsRef<str>,
{
    list_repos_by_category(user_config, category)
        .map(|category| category.uncommitted_changes_only())
}
