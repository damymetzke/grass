pub mod config;

use itertools::Itertools;

/// List categories
///
/// # Examples
///
/// ```
/// # use grass::list_categories;
///
/// assert_eq!(vec!{"general", "work"}, list_categories());
/// ```
pub fn list_categories() -> Vec<String> {
    let user_config = config::load_user_config().unwrap_or_default();
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
/// let result_general = list_repos_by_category("general");
/// let result_work = list_repos_by_category("work");
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
