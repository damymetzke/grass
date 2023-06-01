mod local;

use super::git::GitStrategy;

pub use local::LocalApiStrategy;

pub trait ApiStrategy
{
    type Git: GitStrategy;
    fn get_git_strategy(&self) -> &Self::Git;
}
