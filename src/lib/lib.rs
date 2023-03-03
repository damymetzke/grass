pub mod config;

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
pub fn list_categories(user_config: &config::RootConfig) -> Vec<String> {
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
pub fn list_repos_by_category<T: AsRef<str>>(
    category_name: T,
) -> Option<SimpleCategoryDescription> {
    match category_name.as_ref() {
        "general" | "work" => Some(SimpleCategoryDescription {
            category: String::from(category_name.as_ref()),
            repositories: vec![String::from("first"), String::from("second")],
        }),
        _ => None,
    }
}
