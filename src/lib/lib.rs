pub mod config;

use std::{fs, path::PathBuf};

use config::RootConfig;
use itertools::Itertools;

/// List categories
///
/// # Examples
///
/// ```
/// # use grass::config;
///
/// let user_config = config::load_example_config();
///
/// assert_eq!(vec!{"general", "work"}, grass::list_categories(&user_config));
/// ```
pub fn list_categories(user_config: &RootConfig) -> Vec<String> {
    user_config
        .grass
        .category
        .keys()
        .map(String::from)
        .sorted()
        .collect()
}

pub struct SimpleCategoryDescription {
    pub category: String,
    pub repositories: Vec<String>,
}

fn get_base_directory(user_config: &RootConfig) -> Option<PathBuf> {
    let base_directory = &user_config.grass.base_dir;
    Some(match base_directory.chars().next_tuple() {
        Some(('~', '/')) => dirs::home_dir()?.join(base_directory.get(2..)?),
        _ => PathBuf::from(base_directory),
    })
}

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

    let repositories: Vec<String> = if let Ok(entries) = fs::read_dir(root_folder) {
        entries
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| match entry.metadata() {
                Ok(metadata) if metadata.is_dir() => Some(entry.file_name().into_string().ok()?),
                _ => None,
            })
            .sorted()
            .collect()
    } else {
        return None;
    };

    Some(SimpleCategoryDescription {
        category: String::from(category_name.as_ref()),
        repositories,
    })
}

pub fn list_all_repositories(user_config: &RootConfig) -> Vec<SimpleCategoryDescription> {
    user_config
        .grass
        .category
        .iter()
        .filter_map(|(key, _)| list_repos_by_category(user_config, key))
        .collect()
}

pub fn get_category_path<T>(user_config: &RootConfig, category: T) -> Option<PathBuf>
where
    T: AsRef<str>,
{
    let category = user_config.grass.get_from_category_or_alias(&category)?;

    Some(PathBuf::from(&user_config.grass.base_dir).join(&category.name))
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
