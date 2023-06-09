use crate::{config::GrassConfig, dev::strategy::git::LocalGitStrategy};

use super::ApiStrategy;

pub struct LocalApiStrategy<'a> {
    git_strategy: LocalGitStrategy<'a>,
}

impl<'a> LocalApiStrategy<'a> {
    pub fn new(config: &'a GrassConfig) -> Self {
        LocalApiStrategy {
            git_strategy: LocalGitStrategy::new(config),
        }
    }
}

impl<'a> ApiStrategy for LocalApiStrategy<'a> {
    type Git = LocalGitStrategy<'a>;

    fn get_git_strategy(&self) -> &LocalGitStrategy<'a> {
        &self.git_strategy
    }
}
