use tracing::trace;

use crate::dev::{strategy::git::RepositoryChangeStatus, RepositoryLocation};

pub struct UncommittedChangesOnlyIterator<'a, T> {
    source: &'a mut T,
}

impl<'a, T: Iterator<Item = (RepositoryLocation, RepositoryChangeStatus)>> Iterator
    for UncommittedChangesOnlyIterator<'a, T>
{
    type Item = (RepositoryLocation, RepositoryChangeStatus);

    fn next(&mut self) -> Option<Self::Item> {
        for next in &mut self.source {
            trace!(
                "Considering repository '{}';\nWith change status {}",
                next.0,
                next.1
            );

            if matches!(next.1, RepositoryChangeStatus::UpToDate) {
                continue;
            };

            return Some(next);
        }
        None
    }
}

pub trait LocationAndChangeStatusIterExtensions:
    Iterator<Item = (RepositoryLocation, RepositoryChangeStatus)> + Sized
{
    fn uncommitted_changes_only(&mut self) -> UncommittedChangesOnlyIterator<Self>;
}

impl<T: Iterator<Item = (RepositoryLocation, RepositoryChangeStatus)> + Sized>
    LocationAndChangeStatusIterExtensions for T
{
    fn uncommitted_changes_only(&mut self) -> UncommittedChangesOnlyIterator<Self> {
        UncommittedChangesOnlyIterator { source: self }
    }
}
