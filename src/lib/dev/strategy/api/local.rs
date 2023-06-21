use crate::dev::{
    config::GrassConfig,
    strategy::{discovery::LocalDiscoveryStrategy, git::LocalGitStrategy},
};

use super::ApiStrategy;

pub struct LocalApiStrategy<'a> {
    git_strategy: LocalGitStrategy<'a>,
    discovery_strategy: LocalDiscoveryStrategy<'a>,
}

impl<'a> LocalApiStrategy<'a> {
    pub fn new(config: &'a GrassConfig) -> Self {
        LocalApiStrategy {
            git_strategy: LocalGitStrategy::new(config),
            discovery_strategy: LocalDiscoveryStrategy::new(config),
        }
    }
}

impl<'a> ApiStrategy for LocalApiStrategy<'a> {
    type Git = LocalGitStrategy<'a>;
    type Discovery = LocalDiscoveryStrategy<'a>;

    fn get_git_strategy(&self) -> &Self::Git {
        &self.git_strategy
    }

    fn get_discovery_strategy(&self) -> &Self::Discovery {
        &self.discovery_strategy
    }
}
