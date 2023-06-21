use std::error::Error as StdError;

use thiserror::Error;

use crate::dev::{
    config,
    strategy::{
        api::{ApiStrategy, LocalApiStrategy},
        discovery::LocalDiscoveryStrategy,
        git::LocalGitStrategy,
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

#[derive(Error)]
pub enum LocalStrategyError<T>
where
    T: StdError,
{
    #[error("Could not load user configuration\nReason: {reason}")]
    LoadConfigError { reason: String },
    #[error(transparent)]
    UserError(#[from] T),
}

/// Builds the API strategy using the default local config
///
/// The base configuration directory is located using [dirs::config_dir].
/// All files under the `{config}/grass/` that end with `.toml` are then considered.
/// They are considered in alphabetical order, with later values overriding earlier ones.
pub fn use_local_strategy_with_default_config<T, U, V>(
    closure: T,
) -> Result<U, LocalStrategyError<V>>
where
    T: Fn(Api<LocalApiStrategy>) -> Result<U, LocalStrategyError<V>>,
    V: StdError,
{
    let config = config::load_user_config()
        .map_err(|error| LocalStrategyError::LoadConfigError {
            reason: error.to_string(),
        })?
        .grass;

    let git_strategy = LocalGitStrategy::new(&config);
    let discovery_strategy = LocalDiscoveryStrategy::new(&config);

    let api_strategy = LocalApiStrategy::new(&git_strategy, &discovery_strategy);

    closure(Api(api_strategy))
}
