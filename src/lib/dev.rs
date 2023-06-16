pub mod config;
mod public;
pub mod repository;
pub mod strategy;

pub use public::{
    categories::{get_category_path, list_categories},
    repositories::{
        get_repository, get_repository_path, list_all_repositories,
        list_all_repositories_with_change_status, list_repos_by_category,
        list_repositories_with_change_status,
    },
    types,
};

pub use public::{
    api::{Api, RepositoryLocation},
    git::{
        clean_repository, clone_repository, clone_repository_default, get_repository_change_status,
    },
};