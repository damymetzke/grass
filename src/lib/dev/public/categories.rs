use std::path::PathBuf;

use crate::dev::config::GrassConfig;

// TODO: Return Result instead of Option
#[deprecated]
pub fn get_category_path<T>(user_config: &GrassConfig, category: T) -> Option<PathBuf>
where
    T: AsRef<str>,
{
    let category = user_config.get_from_category_or_alias(&category)?;

    Some(user_config.base_dir.join(&category.name))
}
