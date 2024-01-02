use crate::dev::strategy::{
    alias::{MockAliasStrategy, SupportsAlias},
    discovery::MockDiscoveryStrategy,
    git::MockGitStrategy,
    path::MockPathStrategy,
};

use super::{SupportsDiscovery, SupportsGit, SupportsPath};

#[derive(Default)]
pub struct MockApiStrategy {
    alias_strategy: MockAliasStrategy,
    discovery_strategy: MockDiscoveryStrategy,
    git_strategy: MockGitStrategy,
    path_strategy: MockPathStrategy,
}

impl MockApiStrategy {
    pub fn new() -> Self {
        Self::default()
    }
}

impl SupportsAlias for MockApiStrategy {
    type Strategy = MockAliasStrategy;

    fn get_alias_strategy(&self) -> &Self::Strategy {
        &self.alias_strategy
    }
}

impl SupportsDiscovery for MockApiStrategy {
    type Strategy = MockDiscoveryStrategy;

    fn get_discovery_strategy(&self) -> &Self::Strategy {
        &self.discovery_strategy
    }
}

impl SupportsGit for MockApiStrategy {
    type Strategy = MockGitStrategy;

    fn get_git_strategy(&self) -> &Self::Strategy {
        &self.git_strategy
    }
}

impl SupportsPath for MockApiStrategy {
    type Strategy = MockPathStrategy;

    fn get_path_strategy(&self) -> &Self::Strategy {
        &self.path_strategy
    }
}
