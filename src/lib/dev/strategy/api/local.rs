use crate::dev::strategy::{
    discovery::LocalDiscoveryStrategy, git::LocalGitStrategy, path::LocalPathStrategy,
};

use super::ApiStrategy;

pub struct LocalApiStrategy<'a> {
    discovery_strategy: &'a LocalDiscoveryStrategy<'a>,
    git_strategy: &'a LocalGitStrategy<'a>,
    path_strategy: &'a LocalPathStrategy<'a>,
}

impl<'a> LocalApiStrategy<'a> {
    pub fn new(
        discovery_strategy: &'a LocalDiscoveryStrategy<'a>,
        git_strategy: &'a LocalGitStrategy<'a>,
        path_strategy: &'a LocalPathStrategy<'a>,
    ) -> Self {
        Self {
            discovery_strategy,
            git_strategy,
            path_strategy,
        }
    }
}

impl<'a> ApiStrategy for LocalApiStrategy<'a> {
    type Discovery = LocalDiscoveryStrategy<'a>;
    type Git = LocalGitStrategy<'a>;
    type Path = LocalPathStrategy<'a>;

    fn get_discovery_strategy(&self) -> &Self::Discovery {
        self.discovery_strategy
    }

    fn get_git_strategy(&self) -> &Self::Git {
        self.git_strategy
    }

    fn get_path_strategy(&self) -> &Self::Path {
        self.path_strategy
    }
}
