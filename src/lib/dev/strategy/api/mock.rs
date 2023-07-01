use crate::dev::strategy::{
    discovery::MockDiscoveryStrategy, git::MockGitStrategy, path::MockPathStrategy,
};

use super::{SupportsAll, SupportsDiscovery, SupportsGit, SupportsPath};

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

impl SupportsAll for MockApiStrategy {}

impl SupportsDiscovery for MockApiStrategy {
    type Discovery = MockDiscoveryStrategy;

    fn get_discovery_strategy(&self) -> &Self::Discovery {
        &self.discovery_strategy
    }
}

impl SupportsGit for MockApiStrategy {
    type Git = MockGitStrategy;

    fn get_git_strategy(&self) -> &Self::Git {
        &self.git_strategy
    }
}

impl SupportsPath for MockApiStrategy {
    type Path = MockPathStrategy;

    fn get_path_strategy(&self) -> &Self::Path {
        &self.path_strategy
    }
}
