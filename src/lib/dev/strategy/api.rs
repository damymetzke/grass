mod local;
mod mock;

use super::{
    alias::SupportsAlias, discovery::SupportsDiscovery, git::SupportsGit, path::SupportsPath,
};

pub use local::LocalApiStrategy;
pub use mock::MockApiStrategy;

pub trait SupportsAll: SupportsAlias + SupportsDiscovery + SupportsGit + SupportsPath {}

impl<T: SupportsAlias + SupportsDiscovery + SupportsGit + SupportsPath> SupportsAll for T {}
