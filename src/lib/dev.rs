pub mod config;
pub mod error;
pub mod iterator;
mod public;
pub mod repository;
pub mod strategy;

pub use public::{
    api::{Category, RepositoryLocation},
    changes::{
        get_repository_change_status,
        list_repositories_with_change_status as list_repositories_with_change_status_next,
        list_repositories_with_change_status_in_category,
        list_repositories_with_uncommitted_changes,
        list_repositories_with_uncommitted_changes_in_category, ChangeStatusResult,
    },
    discovery::{
        create_repository, list_all_repositories, list_categories, list_repositories_in_category,
        list_repositories_in_category_with_errors, verify_repository_exists,
    },
    git::{clean_repository, clone_repository, clone_repository_default},
    path::get_repository_path as get_repository_path_next,
    repositories::resolve_repository_alias,
    strategy::{use_local_strategy_with_default_config, use_mock_strategy, Api},
};
