use std::path::PathBuf;

use itertools::Itertools;

use crate::config::RootConfig;

pub fn get_base_directory(user_config: &RootConfig) -> Option<PathBuf> {
    let base_directory = &user_config.grass.base_dir;
    Some(match base_directory.chars().next_tuple() {
        Some(('~', '/')) => dirs::home_dir()?.join(base_directory.get(2..)?),
        _ => PathBuf::from(base_directory),
    })
}
