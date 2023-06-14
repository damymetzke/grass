#![allow(rustdoc::bare_urls)]
mod config;
mod public;
mod repository;
mod strategy;

#[cfg(feature = "dev")]
pub mod dev {
    pub use crate::public::{
        categories::{get_category_path, list_categories},
        repositories::{
            get_repository, get_repository_path, list_all_repositories,
            list_all_repositories_with_change_status, list_repos_by_category,
            list_repositories_with_change_status,
        },
        types,
    };

    pub use crate::public::{
        api::{Api, RepositoryLocation},
        git::{
            clean_repository, clone_repository, clone_repository_default,
            get_repository_change_status,
        },
    };

    pub mod config {
        pub use crate::config::*;
    }

    pub mod repository {
        pub use crate::repository::*;
    }

    pub mod strategy {
        pub use crate::strategy::*;
    }
}
