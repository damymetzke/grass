use crate::dev::{
    strategy::git::RepositoryChangeStatus,
    RepositoryLocation,
};

pub struct UncommittedChangesOnlyIterator<'a, T> {
    source: &'a mut T,
}

impl<'a, T: Iterator<Item = (RepositoryLocation, RepositoryChangeStatus)>> Iterator
    for UncommittedChangesOnlyIterator<'a, T>
{
    type Item = (RepositoryLocation, RepositoryChangeStatus);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = match self.source.next() {
                Some(next) => next,
                None => break, // Iteration complete, break loop and return None
            };

            if matches!(next.1, RepositoryChangeStatus::UpToDate) {
                continue;
            };

            Some(next);
        }
        None
    }
}

pub trait LocationAndChangeStatusIterExtensions:
    Iterator<Item = (RepositoryLocation, RepositoryChangeStatus)> + Sized
{
    fn uncommitted_changes_only<'a>(&'a mut self) -> UncommittedChangesOnlyIterator<'a, Self>;
}

impl<T: Iterator<Item = (RepositoryLocation, RepositoryChangeStatus)> + Sized>
    LocationAndChangeStatusIterExtensions for T
{
    fn uncommitted_changes_only<'a>(&'a mut self) -> UncommittedChangesOnlyIterator<'a, Self> {
        UncommittedChangesOnlyIterator { source: self }
    }
}
