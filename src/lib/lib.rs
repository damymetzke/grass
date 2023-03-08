mod commands;
pub mod config;

use std::path::PathBuf;

use config::RootConfig;
use itertools::Itertools;

pub use commands::{
    categories::{get_category_path, list_categories},
    repositories::{get_repository_path, list_all_repositories, list_repos_by_category},
    types,
};

fn get_base_directory(user_config: &RootConfig) -> Option<PathBuf> {
    let base_directory = &user_config.grass.base_dir;
    Some(match base_directory.chars().next_tuple() {
        Some(('~', '/')) => dirs::home_dir()?.join(base_directory.get(2..)?),
        _ => PathBuf::from(base_directory),
    })
}
