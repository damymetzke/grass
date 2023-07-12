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
pub enum AliasStrategyError {
    UnkownError { context: String, reason: String },
}

/// Alias for methods of `AliasStrategy`.
pub type Result<T> = std::result::Result<T, AliasStrategyError>;

/// Describes an alias between
pub struct Alias {
    pub alias: String,
    pub category: String,
}

/// The result from `AliasStrategy::resolve_alias`[^resolve]
///
/// [^resolve]: [crate::dev::strategy::alias::AliasStrategy::resolve_alias]
pub enum ResolveAliasResult {
    Alias(Alias),
    NoAlias(String),
}

/// Strategy for resolving and listing aliases.
///
/// # Todo:
///
/// - [ ] Add examples to methods
pub trait AliasStrategy {
    /// Get a list of all aliases
    fn list_all_aliases<T>(&self) -> Result<T>
    where
        T: FromIterator<Alias>;

    /// Get a list of aliases for a specific category
    fn list_aliases_for_category<T, U>(&self, category: T) -> Result<U>
    where
        T: AsRef<str>,
        U: FromIterator<Alias>;

    /// Returns the alias if it exists.
    ///
    /// This function is unaware of categories.
    fn resolve_alias<T>(&self, alias: T) -> Result<ResolveAliasResult>
    where
        T: AsRef<str>;
}
