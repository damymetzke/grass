use crate::dev::strategy::git::MockGitStrategy;

use super::ApiStrategy;

#[derive(Default)]
pub struct MockApiStrategy {
    git_strategy: MockGitStrategy,
}

impl MockGitStrategy {
    pub fn new() -> Self {
        Self::default()
    }
}

impl ApiStrategy for MockApiStrategy {
    type Git = MockGitStrategy;

    fn get_git_strategy(&self) -> &Self::Git {
        &self.git_strategy
    }
}
