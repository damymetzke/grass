mod local;
mod mock;

use super::git::GitStrategy;

pub use local::LocalApiStrategy;
pub use mock::MockApiStrategy;

pub trait ApiStrategy {
    type Git: GitStrategy;
    fn get_git_strategy(&self) -> &Self::Git;
}
