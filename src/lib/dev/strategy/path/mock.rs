use std::path::PathBuf;

use crate::dev::RepositoryLocation;

use super::{PathStrategy, PathStrategyError};

/// A mocked implementation of the path strategy
///
/// TODO: Get a central documentation for mocked data
pub struct MockPathStrategy;

impl PathStrategy for MockPathStrategy {
    fn get_containing_directory<T>(&self, repository: T) -> super::Result<std::path::PathBuf>
    where
        T: Into<crate::dev::RepositoryLocation>,
    {
        let repository: RepositoryLocation = repository.into();
        let repository = (repository.category.as_str(), repository.repository.as_str());
        match repository {
            ("all_good", _) => Ok(PathBuf::from("/home/example/repositories")),
            ("with_changes", _) => Ok(PathBuf::from("/home/example/repositories/with_changes")),
            ("with_error", _) => Ok(PathBuf::from("/home/example/repositories/with_error")),
            _ => Err(PathStrategyError::RepositoryNotFound {
                context: "When mocking".into(),
                reason: "Category not found".into(),
            }),
        }
    }

    fn get_directory<T>(&self, repository: T) -> super::Result<std::path::PathBuf>
    where
        T: Into<crate::dev::RepositoryLocation>,
    {
        let repository: RepositoryLocation = repository.into();
        let repository = (repository.category.as_str(), repository.repository.as_str());
        match repository {
            ("all_good", "first") => Ok(PathBuf::from("/home/example/repositories/all_good/first")),
            ("all_good", "second") => {
                Ok(PathBuf::from("/home/example/repositories/all_good/second"))
            }
            ("all_good", "third") => Ok(PathBuf::from("/home/example/repositories/all_good/third")),
            ("with_changes", "first") => Ok(PathBuf::from(
                "/home/example/repositories/with_changes/first",
            )),
            ("with_changes", "second") => Ok(PathBuf::from(
                "/home/example/repositories/with_changes/second",
            )),
            ("with_changes", "third") => Ok(PathBuf::from(
                "/home/example/repositories/with_changes/third",
            )),
            ("with_error", "first") => {
                Ok(PathBuf::from("/home/example/repositories/with_error/first"))
            }
            ("with_error", "second") => Err(PathStrategyError::Unknown {
                context: "When mocking".into(),
                reason: "Insufficient permissions".into(),
            }),
            ("all_good" | "with_changes" | "with_error", _) => {
                Err(PathStrategyError::RepositoryNotFound {
                    context: "When mocking".into(),
                    reason: "Repository not found".into(),
                })
            }
            _ => Err(PathStrategyError::RepositoryNotFound {
                context: "When mocking".into(),
                reason: "Category not found".into(),
            }),
        }
    }
}
