mod local;
mod mock;

use super::{discovery::DiscoveryStrategy, git::GitStrategy, path::PathStrategy};

pub use local::LocalApiStrategy;
pub use mock::MockApiStrategy;

pub trait SupportsAll: SupportsDiscovery + SupportsGit + SupportsPath {}

pub trait SupportsDiscovery {
    type Discovery: DiscoveryStrategy;

    fn get_discovery_strategy(&self) -> &Self::Discovery;
}

pub trait SupportsGit {
    type Git: GitStrategy;

    fn get_git_strategy(&self) -> &Self::Git;
}

pub trait SupportsPath {
    type Path: PathStrategy;

    fn get_path_strategy(&self) -> &Self::Path;
}
