use crate::dev::strategy::{discovery::LocalDiscoveryStrategy, git::LocalGitStrategy};

use super::ApiStrategy;

pub struct LocalApiStrategy<'a> {
    git_strategy: &'a LocalGitStrategy<'a>,
    discovery_strategy: &'a LocalDiscoveryStrategy<'a>,
}

impl<'a> LocalApiStrategy<'a> {
    pub fn new(
        git_strategy: &'a LocalGitStrategy<'a>,
        discovery_strategy: &'a LocalDiscoveryStrategy<'a>,
    ) -> Self {
        Self {
            git_strategy,
            discovery_strategy,
        }
    }
}

impl<'a> ApiStrategy for LocalApiStrategy<'a> {
    type Git = LocalGitStrategy<'a>;
    type Discovery = LocalDiscoveryStrategy<'a>;

    fn get_git_strategy(&self) -> &Self::Git {
        self.git_strategy
    }

    fn get_discovery_strategy(&self) -> &Self::Discovery {
        self.discovery_strategy
    }
}
