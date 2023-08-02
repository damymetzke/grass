use thiserror::Error;

use super::strategy::{
    alias::AliasStrategyError, discovery::DiscoveryStrategyError, git::GitStrategyError,
    path::PathStrategyError,
};

#[derive(Debug, Error)]
pub enum GrassError {
    #[error(transparent)]
    AliasStrategy(#[from] AliasStrategyError),
    #[error(transparent)]
    DiscoveryStrategy(#[from] DiscoveryStrategyError),
    #[error(transparent)]
    GitStrategy(#[from] GitStrategyError),
    #[error(transparent)]
    PathStrategy(#[from] PathStrategyError),
}
