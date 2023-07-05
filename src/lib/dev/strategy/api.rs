mod local;
mod mock;

use super::{discovery::SupportsDiscovery, git::SupportsGit, path::SupportsPath};

pub use local::LocalApiStrategy;
pub use mock::MockApiStrategy;

pub trait SupportsAll: SupportsDiscovery + SupportsGit + SupportsPath {}

impl<T: SupportsDiscovery + SupportsGit + SupportsPath> SupportsAll for T {}
