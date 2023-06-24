//! Collection of strategies used in the crate
//!
//! All operations like git operations and file access is abstracted by strategies.
//! This is core to the design of the crate.
//!
//! There are 3 distinct reasons why these strategies exist:
//!
//! # Motivation
//!
//! - Future mobility, this will simplify changing behavior or adding extra modes.
//! - Third party flexibility, you have the option to override this behavior for different
//!   platforms.
//! - Testing, this makes API tests work by mocing the behavior of these strategies.
//!
//! The most important reason is testing.
//! All API points must be documented with an example,
//! the mocked strategies simply supply the means to do so more easily.
//!
//! # Overview
//!
//! At the center is the `ApiStrategy`[^1].
//! This is a collection of all strategies.
//! This is done to support the following logic:
//!
//! 1. Build an `ApiStrategy`[^1] using all required strategies.
//! 2. Provide this strategy as an input to all usages of the public API.
//! 3. The API uses the provided strategies.
//!
//! Step 1 is made simpler by providing default methods.
//! Currently these are the following:
//!
//! - TODO: Link to the function for the local strategy once created.
//! - TODO: Link to the function for the mocked strategy once created.
//!
//! The other strategies have the following purposes:
//!
//! | strategy                                             | description                                      |
//! | :--------------------------------------------------- | :----------------------------------------------- |
//! | [crate::dev::strategy::discovery::DiscoveryStrategy] | List and find repositories, independent of paths |
//! | [crate::dev::strategy::git::GitStrategy]             | Read and write operations using Git              |
//! | [crate::dev::strategy::path::PathStrategy]           | Resolve repositories to file system paths        |
//!
//! [^1]: [crate::dev::strategy::api::ApiStrategy]

pub mod api;
pub mod discovery;
pub mod git;
pub mod path;
