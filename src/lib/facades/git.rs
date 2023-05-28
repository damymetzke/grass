use git2::Repository;
use itertools::Itertools;
use std::{fmt::Display, path::Path};
use thiserror::Error;

#[derive(Error, Debug)]
pub struct FileSystemErrorCollection(Vec<std::io::Error>);

impl Display for FileSystemErrorCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "There are {} file system errors:\n{}",
            self.0.len(),
            self.0.iter().map(|item| item.to_string()).join("\n")
        )
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Git2(#[from] git2::Error),
    #[error(transparent)]
    FileSystem(FileSystemErrorCollection),
}

impl Error {
    pub fn from_file_system_errors(errors: Vec<std::io::Error>) -> Result<()> {
        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::FileSystem(FileSystemErrorCollection(errors)))
        }
    }
}

type Result<T> = std::result::Result<T, Error>;

pub fn clean_repository<T>(repository_path: T) -> Result<()>
where
    T: AsRef<Path>,
{
    let repository = Repository::open(repository_path.as_ref())?;
    let statuses = repository.statuses(None)?;
    let results: Vec<_> = statuses
        .iter()
        .filter_map(|entry| match entry.path() {
            Some(file_path) if entry.status().is_ignored() => Some(file_path.to_owned()),
            _ => None,
        })
        .map(|file| {
            let path = repository_path.as_ref().join(file);
            if path.is_dir() {
                std::fs::remove_dir_all(path)
            } else {
                std::fs::remove_file(path)
            }
        })
        .filter_map(std::result::Result::err)
        .collect();

    Error::from_file_system_errors(results)
}
