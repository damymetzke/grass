mod local;
mod mock;

use super::{git::GitStrategy, discovery::DiscoveryStrategy};

pub use local::LocalApiStrategy;
pub use mock::MockApiStrategy;

pub trait ApiStrategy {
    type Git: GitStrategy;
    type Discovery: DiscoveryStrategy;

    fn get_git_strategy(&self) -> &Self::Git;
    fn get_discovery_strategy(&self) -> &Self::Discovery;
}
