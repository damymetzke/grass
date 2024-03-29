use thiserror::Error;
use tracing::info;

use crate::dev::{
    config,
    strategy::{
        alias::LocalAliasStrategy,
        api::{LocalApiStrategy, MockApiStrategy},
        discovery::LocalDiscoveryStrategy,
        git::LocalGitStrategy,
        path::LocalPathStrategy,
    },
};

#[derive(Debug)]
pub struct Api<T>(T);

pub trait AccessApi<T> {
    fn get_strategy(&self) -> &T;
}

impl<T> Api<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> From<T> for Api<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Clone> From<&T> for Api<T> {
    fn from(value: &T) -> Self {
        Self(value.clone())
    }
}

impl<T> AccessApi<T> for Api<T> {
    fn get_strategy(&self) -> &T {
        &self.0
    }
}

#[macro_export]
macro_rules! support_strategy {
    ($trait_name:ident, $method_name:ident, $strategy_type:ident) => {
        pub trait $trait_name {
            type Strategy: $strategy_type;

            fn $method_name(&self) -> &Self::Strategy;
        }

        impl<T> $crate::dev::public::strategy::Api<T>
        where
            T: $trait_name,
        {
            pub(crate) fn $method_name(&self) -> &T::Strategy {
                $crate::dev::public::strategy::AccessApi::get_strategy(self).$method_name()
            }
        }
    };
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

    let alias_strategy = LocalAliasStrategy::new(&config);
    let path_strategy = LocalPathStrategy::new(&config);
    let discovery_strategy = LocalDiscoveryStrategy::new(&config, &path_strategy);
    let git_strategy = LocalGitStrategy::new(&path_strategy);

    let api_strategy = LocalApiStrategy::new(
        &alias_strategy,
        &discovery_strategy,
        &git_strategy,
        &path_strategy,
    );

    info!("Using local strategy with default config");
    closure(api_strategy.into())
}

/// Builds the API strategy using the mocking strategy
///
/// This can be used during testing.
pub fn use_mock_strategy() -> Api<MockApiStrategy> {
    Api(MockApiStrategy::default())
}
