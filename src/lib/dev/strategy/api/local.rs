use crate::dev::strategy::{
    discovery::LocalDiscoveryStrategy, git::LocalGitStrategy, path::LocalPathStrategy,
};

use super::{ApiStrategy, HasDiscoveryStrategy, HasGitStrategy, HasPathStrategy};

pub struct LocalApiStrategy<'a> {
    discovery_strategy: &'a LocalDiscoveryStrategy<'a, LocalPathStrategy<'a>>,
    git_strategy: &'a LocalGitStrategy<'a, LocalPathStrategy<'a>>,
    path_strategy: &'a LocalPathStrategy<'a>,
}

impl<'a> LocalApiStrategy<'a> {
    pub fn new(
        discovery_strategy: &'a LocalDiscoveryStrategy<'a, LocalPathStrategy<'a>>,
        git_strategy: &'a LocalGitStrategy<'a, LocalPathStrategy<'a>>,
        path_strategy: &'a LocalPathStrategy<'a>,
    ) -> Self {
        Self {
            discovery_strategy,
            git_strategy,
            path_strategy,
        }
    }
}

impl<'a> ApiStrategy for LocalApiStrategy<'a> {}

impl<'a> HasDiscoveryStrategy for LocalApiStrategy<'a> {
    type Discovery = LocalDiscoveryStrategy<'a, LocalPathStrategy<'a>>;

    fn get_discovery_strategy(&self) -> &Self::Discovery {
        self.discovery_strategy
    }
}

impl<'a> HasGitStrategy for LocalApiStrategy<'a> {
    type Git = LocalGitStrategy<'a, LocalPathStrategy<'a>>;

    fn get_git_strategy(&self) -> &Self::Git {
        self.git_strategy
    }
}

impl<'a> HasPathStrategy for LocalApiStrategy<'a> {
    type Path = LocalPathStrategy<'a>;

    fn get_path_strategy(&self) -> &Self::Path {
        self.path_strategy
    }
}
