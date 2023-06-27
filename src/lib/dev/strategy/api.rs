mod local;
mod mock;

use super::{discovery::DiscoveryStrategy, git::GitStrategy, path::PathStrategy};

pub use local::LocalApiStrategy;
pub use mock::MockApiStrategy;

pub trait ApiStrategy {
    type Discovery: DiscoveryStrategy;
    type Git: GitStrategy;
    type Path: PathStrategy;

    fn get_discovery_strategy(&self) -> &Self::Discovery;
    fn get_git_strategy(&self) -> &Self::Git;
    fn get_path_strategy(&self) -> &Self::Path;
}
