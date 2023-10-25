use std::path::PathBuf;

use thiserror::Error;

use crate::dev::{
    public::strategy::AccessApi,
    strategy::{
        alias::{AliasStrategy, AliasStrategyError, SupportsAlias},
        path::{PathStrategy, PathStrategyError, SupportsPath},
    },
    Api, RepositoryLocation,
};

/// Either a path or an alias error
///
/// This should be replaced when
/// <https://github.com/damymetzke/grass/issues/2>
/// is resolved.
#[derive(Debug, Error)]
pub enum PathOrAliasError {
    #[error(transparent)]
    Path(#[from] PathStrategyError),
    #[error(transparent)]
    Alias(#[from] AliasStrategyError),
}

/// Get the path to a repository.
///
/// Also works if the repository does not exist (yet).
/// While this will return a valid path, it is not guarenteed that the path exists.
///
/// # Todo:
///
/// - [ ] Return a generic crate wide error
///       (<https://github.com/damymetzke/grass/issues/2>)
///
/// # Example
///
/// ```rust
/// # use grass::dev::{
/// #     self,
/// #     strategy::{alias::SupportsAlias, api::MockApiStrategy, path::SupportsPath},
/// #     Api,
/// # };
/// # use std::path::PathBuf;
/// # let api = Api::from(MockApiStrategy::default());
/// #
/// fn test_public<T: SupportsPath + SupportsAlias>(api: &Api<T>) {
///     assert_eq!(
///         dev::get_repository_path_next(api, ("all_good", "first")).unwrap(),
///         PathBuf::from("/home/example/repositories/all_good/first")
///     );
/// }
///
/// test_public(&api);
/// ```
pub fn get_repository_path<T: SupportsPath + SupportsAlias, U: Into<RepositoryLocation>>(
    api: &Api<T>,
    repository: U,
) -> Result<PathBuf, PathOrAliasError> {
    let api = api.get_strategy();
    let path = api.get_path_strategy();
    let alias = api.get_alias_strategy();

    let result = path.get_directory(alias.resolve_location_alias(repository)?)?;

    Ok(result)
}
