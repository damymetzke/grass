use crate::dev::strategy::{
    alias::{LocalAliasStrategy, SupportsAlias},
    discovery::LocalDiscoveryStrategy,
    git::LocalGitStrategy,
    path::LocalPathStrategy,
};

use super::{SupportsDiscovery, SupportsGit, SupportsPath};

pub struct LocalApiStrategy<'a> {
    alias_strategy: &'a LocalAliasStrategy<'a>,
    discovery_strategy: &'a LocalDiscoveryStrategy<'a, LocalPathStrategy<'a>>,
    git_strategy: &'a LocalGitStrategy<'a, LocalPathStrategy<'a>>,
    path_strategy: &'a LocalPathStrategy<'a>,
}

impl<'a> LocalApiStrategy<'a> {
    pub fn new(
        api_strategy: &'a LocalAliasStrategy<'a>,
        discovery_strategy: &'a LocalDiscoveryStrategy<'a, LocalPathStrategy<'a>>,
        git_strategy: &'a LocalGitStrategy<'a, LocalPathStrategy<'a>>,
        path_strategy: &'a LocalPathStrategy<'a>,
    ) -> Self {
        Self {
            alias_strategy: api_strategy,
            discovery_strategy,
            git_strategy,
            path_strategy,
        }
    }
}

impl<'a> SupportsDiscovery for LocalApiStrategy<'a> {
    type Discovery = LocalDiscoveryStrategy<'a, LocalPathStrategy<'a>>;

    fn get_discovery_strategy(&self) -> &Self::Discovery {
        self.discovery_strategy
    }
}

impl<'a> SupportsGit for LocalApiStrategy<'a> {
    type Git = LocalGitStrategy<'a, LocalPathStrategy<'a>>;

    fn get_git_strategy(&self) -> &Self::Git {
        self.git_strategy
    }
}

impl<'a> SupportsPath for LocalApiStrategy<'a> {
    type Path = LocalPathStrategy<'a>;

    fn get_path_strategy(&self) -> &Self::Path {
        self.path_strategy
    }
}

impl<'a> SupportsAlias for LocalApiStrategy<'a> {
    type Alias = LocalAliasStrategy<'a>;

    fn get_alias_strategy(&self) -> &Self::Alias {
        self.alias_strategy
    }
}
