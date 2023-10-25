mod local;
mod mock;
mod nop;
mod resolves;

pub use local::LocalAliasStrategy;
pub use mock::MockAliasStrategy;
pub use nop::NopAliasStrategy;
pub use resolves::ResolvesAlias;
use thiserror::Error;

use crate::dev::{public::api::Category, RepositoryLocation};

/// Error returned by methods of `AliasStrategy`[^strategy].
///
/// # Fields
///
/// Each variant has at least the following fields:
///
/// - `context`: What action was attempted.
/// - `reason`: What went wrong, often provided by third party crates.
///
/// [^strategy]: [crate::dev::strategy::alias::AliasStrategy]
#[derive(Debug, Error, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AliasStrategyError {
    #[error("There is a problem:\nContext: {context}\nReason: {reason}")]
    UnkownError { context: String, reason: String },
    #[error("Cannot find category:\nContext: {context}\nReason: {reason}")]
    CategoryNotFound { context: String, reason: String },
}

/// Alias for methods of `AliasStrategy`.
pub type Result<T> = std::result::Result<T, AliasStrategyError>;

/// Describes an alias between
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Alias {
    pub alias: String,
    pub category: Category,
}

/// The result from `AliasStrategy::resolve_alias`[^resolve]
///
/// [^resolve]: [crate::dev::strategy::alias::AliasStrategy::resolve_alias]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResolveAliasResult {
    Alias(Alias),
    NoAlias(String),
}

/// Strategy for resolving and listing aliases.
pub trait AliasStrategy {
    /// Get a list of all aliases
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grass::dev::strategy::alias::{Alias, AliasStrategy, MockAliasStrategy};
    /// #
    /// # let strategy = MockAliasStrategy;
    /// #
    /// fn test_strategy<T: AliasStrategy>(strategy: &T) {
    ///     assert_eq!(
    ///         strategy.list_all_aliases(),
    ///         Ok(vec![
    ///             Alias {
    ///                 alias: "allg".into(),
    ///                 category: "all_good".into()
    ///             },
    ///             Alias {
    ///                 alias: "change".into(),
    ///                 category: "with_changes".into()
    ///             },
    ///             Alias {
    ///                 alias: "err".into(),
    ///                 category: "with_error".into()
    ///             },
    ///         ])
    ///     )
    /// }
    ///
    /// test_strategy(&strategy)
    /// ```
    fn list_all_aliases<T>(&self) -> Result<T>
    where
        T: FromIterator<Alias>;

    /// Get a list of aliases for a specific category
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grass::dev::strategy::alias::{Alias, AliasStrategy, MockAliasStrategy};
    /// #
    /// # let strategy = MockAliasStrategy;
    /// #
    /// fn test_strategy<T: AliasStrategy>(strategy: &T) {
    ///     assert_eq!(
    ///         strategy.list_aliases_for_category("all_good"),
    ///         Ok(vec![Alias {
    ///             alias: "allg".into(),
    ///             category: "all_good".into()
    ///         }])
    ///     );
    /// }
    ///
    /// test_strategy(&strategy)
    /// ```
    fn list_aliases_for_category<T, U>(&self, category: T) -> Result<U>
    where
        T: AsRef<str>,
        U: FromIterator<Alias>;

    /// Returns the alias if it exists.
    ///
    /// This function is unaware of categories.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grass::dev::strategy::alias::{
    /// #     Alias, AliasStrategy, MockAliasStrategy, ResolveAliasResult,
    /// # };
    /// #
    /// # let strategy = MockAliasStrategy;
    /// #
    /// fn test_strategy<T: AliasStrategy>(strategy: &T) {
    ///     assert_eq!(
    ///         strategy.resolve_alias_old("allg"),
    ///         Ok(ResolveAliasResult::Alias(Alias {
    ///             alias: "allg".into(),
    ///             category: "all_good".into()
    ///         })),
    ///     );
    ///
    ///     assert_eq!(
    ///         strategy.resolve_alias_old("mispel"),
    ///         Ok(ResolveAliasResult::NoAlias("mispel".into()),)
    ///     );
    /// }
    ///
    /// test_strategy(&strategy)
    /// ```
    #[deprecated = "Use resolve_alias instead"]
    fn resolve_alias_old<T>(&self, alias: T) -> Result<ResolveAliasResult>
    where
        T: AsRef<str>;

    /// Resolve the alias in a RepositoryLocation
    ///
    /// Wraps `resolve_alias`[^resolve].
    /// This will resolve the `category` part of the location.
    /// The repository will not be changed.
    ///
    /// [^resolve]: [crate::dev::strategy::alias::AliasStrategy::resolve_alias]
    #[deprecated = "Use resolve_alias instead"]
    fn resolve_location_alias<T>(&self, location: T) -> Result<RepositoryLocation>
    where
        T: Into<RepositoryLocation>,
    {
        let RepositoryLocation {
            category,
            repository,
        } = location.into();

        Ok(RepositoryLocation {
            category: self.resolve_alias_old(category)?.into(),
            repository,
        })
    }

    /// Resolves the alias of a type
    ///
    /// The input must implement 'ResolvesAlias'[^resolve].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grass::dev::strategy::alias::{
    /// #     Alias, AliasStrategy, MockAliasStrategy, ResolveAliasResult,
    /// # };
    /// #
    /// # let strategy = MockAliasStrategy;
    /// #
    /// fn test_strategy<T: AliasStrategy>(strategy: &T) {
    ///     assert_eq!(
    ///         strategy.resolve_alias("allg"),
    ///         Ok(Box::from("all_good")),
    ///     );
    ///
    ///     assert_eq!(
    ///         strategy.resolve_alias("mispel"),
    ///         Ok(Box::from("mispel")),
    ///     );
    /// }
    ///
    /// test_strategy(&strategy)
    /// ```
    ///
    /// [^resolve]: [crate::dev::strategy::alias::ResolvesAlias]
    fn resolve_alias<T: ResolvesAlias>(&self, input: T) -> Result<T::Resolved>;
}

pub trait SupportsAlias {
    type Alias: AliasStrategy;

    fn get_alias_strategy(&self) -> &Self::Alias;
}

impl<T: Into<String>, U: Into<Category>> From<(T, U)> for Alias {
    fn from(value: (T, U)) -> Self {
        let (alias, category) = value;

        let alias = alias.into();
        let category = category.into();

        Alias { alias, category }
    }
}
