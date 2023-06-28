use thiserror::Error;

use crate::dev::{
    config,
    strategy::{
        api::{ApiStrategy, LocalApiStrategy, MockApiStrategy},
        discovery::LocalDiscoveryStrategy,
        git::LocalGitStrategy,
        path::LocalPathStrategy,
    },
};

pub struct Api<T: ApiStrategy>(T);

pub trait AccessApi<T: ApiStrategy> {
    fn get_strategy(&self) -> &T;
}

impl<T: ApiStrategy> AccessApi<T> for Api<T> {
    fn get_strategy(&self) -> &T {
        &self.0
    }
}

#[derive(Error, Debug)]
pub enum LocalStrategyError {
    #[error("Could not load user configuration\nReason: {reason}")]
    LoadConfigError { reason: String },
}

/// Builds the API strategy using the default local config
///
/// The base configuration directory is located using [dirs::config_dir].
/// All files under the `{config}/grass/` that end with `.toml` are then considered.
/// They are considered in alphabetical order, with later values overriding earlier ones.
pub fn use_local_strategy_with_default_config<T, U>(closure: T) -> Result<U, LocalStrategyError>
where
    T: Fn(Api<LocalApiStrategy>) -> Result<U, LocalStrategyError>,
{
    let config = config::load_user_config()
        .map_err(|error| LocalStrategyError::LoadConfigError {
            reason: error.to_string(),
        })?
        .grass;

    let path_strategy = LocalPathStrategy::new(&config);
    let discovery_strategy = LocalDiscoveryStrategy::new(&config, &path_strategy);
    let git_strategy = LocalGitStrategy::new(&path_strategy);

    let api_strategy = LocalApiStrategy::new(&discovery_strategy, &git_strategy, &path_strategy);

    closure(Api(api_strategy))
}

/// Builds the API strategy using the mocking strategy
///
/// This can be used during testing.
pub fn use_mock_strategy() -> Api<MockApiStrategy> {
    Api(MockApiStrategy::default())
}
