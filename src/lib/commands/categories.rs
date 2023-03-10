use std::path::PathBuf;

use itertools::Itertools;

use crate::{config::RootConfig, util};

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

pub fn get_category_path<T>(user_config: &RootConfig, category: T) -> Option<PathBuf>
where
    T: AsRef<str>,
{
    let category = user_config.grass.get_from_category_or_alias(&category)?;

    Some(
        util::get_base_directory(user_config)
            .map(|path| path.join(&category.name))
            .unwrap_or_default(),
    )
}
