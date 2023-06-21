use crate::dev::strategy::{git::MockGitStrategy, discovery::MockDiscoveryStrategy};

use super::ApiStrategy;

#[derive(Default)]
pub struct MockApiStrategy {
    git_strategy: MockGitStrategy,
    discovery_strategy: MockDiscoveryStrategy,
}

impl MockGitStrategy {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ApiStrategy for MockApiStrategy {
    type Git = MockGitStrategy;
    type Discovery = MockDiscoveryStrategy;

    fn get_git_strategy(&self) -> &Self::Git {
        &self.git_strategy
    }


    fn get_discovery_strategy(&self) -> &Self::Discovery {
        &self.discovery_strategy
    }
}
