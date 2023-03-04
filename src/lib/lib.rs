pub mod config;

use std::fs;

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
    user_config.grass.category.get(category_name.as_ref())?;
    let root_folder = dirs::home_dir()?.join("repos").join(category_name.as_ref());

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
