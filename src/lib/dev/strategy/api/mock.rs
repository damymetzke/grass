use crate::dev::strategy::{
    discovery::MockDiscoveryStrategy, git::MockGitStrategy, path::MockPathStrategy,
};

use super::{ApiStrategy, HasDiscoveryStrategy, HasGitStrategy, HasPathStrategy};

#[derive(Default)]
pub struct MockApiStrategy {
    discovery_strategy: MockDiscoveryStrategy,
    git_strategy: MockGitStrategy,
    path_strategy: MockPathStrategy,
}

impl MockGitStrategy {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ApiStrategy for MockApiStrategy {}

impl HasDiscoveryStrategy for MockApiStrategy {
    type Discovery = MockDiscoveryStrategy;

    fn get_discovery_strategy(&self) -> &Self::Discovery {
        &self.discovery_strategy
    }
}

impl HasGitStrategy for MockApiStrategy {
    type Git = MockGitStrategy;

    fn get_git_strategy(&self) -> &Self::Git {
        &self.git_strategy
    }
}

impl HasPathStrategy for MockApiStrategy {
    type Path = MockPathStrategy;

    fn get_path_strategy(&self) -> &Self::Path {
        &self.path_strategy
    }
}
