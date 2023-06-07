use std::path::PathBuf;

use itertools::Itertools;

use crate::config::{RootConfig, GrassConfig};

/// List categories
///
/// # Examples
///
/// ```
/// # use grass::dev::config;
///
/// let user_config = config::load_example_config();
///
/// // TODO: support dependency injection so this can be tested
/// // assert_eq!(vec!{"general", "work"}, grass::list_categories(&user_config));
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

// TODO: Return Result instead of Option
pub fn get_category_path<T>(user_config: &GrassConfig, category: T) -> Option<PathBuf>
where
    T: AsRef<str>,
{
    let category = user_config.get_from_category_or_alias(&category)?;

    Some(user_config.base_dir.join(&category.name))
}
