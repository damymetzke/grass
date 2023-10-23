use tracing::warn;

use crate::dev::{
    get_repository_change_status,
    strategy::{
        alias::SupportsAlias,
        git::{RepositoryChangeStatus, SupportsGit},
    },
    Api, RepositoryLocation,
};

pub struct WithChangeStatusIterator<'a, T, U>
where
    T: Iterator<Item = RepositoryLocation>,
    U: SupportsGit + SupportsAlias,
{
    source: &'a mut T,
    api: &'a Api<U>,
}

impl<'a, T: Iterator<Item = RepositoryLocation>, U: SupportsGit + SupportsAlias> Iterator
    for WithChangeStatusIterator<'a, T, U>
{
    type Item = (RepositoryLocation, RepositoryChangeStatus);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = match self.source.next() {
                Some(next) => next,
                None => break, // Iteration complete, break loop and return None
            };

            let change_status = match get_repository_change_status(self.api, next.clone()) {
                Ok(change_status) => change_status,
                Err(error) => {
                    warn!(
                        "Could not get the repository change status of repository {}\nReason:\n{}",
                        next, error
                    );
                    continue;
                }
            };

            return Some((next, change_status));
        }
        None
    }
}

pub struct UncommittedChangesOnlyIterator<'a, T> {
    source: &'a mut T,
}

impl<
        'a,
        T: Iterator<Item = (RepositoryLocation, RepositoryChangeStatus)>,
    > Iterator for UncommittedChangesOnlyIterator<'a, T>
{
    type Item = (RepositoryLocation, RepositoryChangeStatus);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = match self.source.next() {
                Some(next) => next,
                None => break, // Iteration complete, break loop and return None
            };

            if matches!(next.1, RepositoryChangeStatus::UpToDate) {
                continue
            };

            Some(next);
        }
        None
    }
}

pub trait LocationIterExtensions: Iterator<Item = RepositoryLocation> + Sized {
    fn with_change_status<'a, T: SupportsGit + SupportsAlias>(
        &'a mut self,
        api: &'a Api<T>,
    ) -> WithChangeStatusIterator<'a, Self, T>;
}

impl<T: Iterator<Item = RepositoryLocation> + Sized> LocationIterExtensions for T {
    fn with_change_status<'a, U: SupportsGit + SupportsAlias>(
        &'a mut self,
        api: &'a Api<U>,
    ) -> WithChangeStatusIterator<'a, Self, U> {
        WithChangeStatusIterator { source: self, api }
    }
}
