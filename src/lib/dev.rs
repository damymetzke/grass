pub mod config;
mod public;
pub mod repository;
pub mod strategy;

pub use public::{
    categories::get_category_path,
    repositories::{
        get_repository, get_repository_path, list_all_repositories_with_change_status,
        list_repositories_with_change_status,
    },
    types,
};

pub use public::{
    api::{RepositoryLocation, Category},
    discovery::{
        list_all_repositories, list_categories, list_repositories_in_category,
        list_repositories_in_category_with_errors,
    },
    git::{
        clean_repository, clone_repository, clone_repository_default, get_repository_change_status,
    },
    strategy::{use_local_strategy_with_default_config, use_mock_strategy, Api},
};
