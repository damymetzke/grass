//! Collection of strategies used in the crate
//!
//! These strategies abstract away the implementation,
//! such that you have full control of how the crate interacts with the system.
//! It is also extensively used for testing.
//!
//! In most cases you should not use these directily when consuming this crate.
//! Instead use the provided `LocalApiStrategy`[^local_api].
//! This fully implements all strategies, using the local system.
//!
//! You *may* change the specific behavior of the crate,
//! by extending any of the strategies.
//! See below for an overview on how to create an `Api`[^api]
//!
//! # Overview
//!
//! To use the strategies for this crate, take the following steps.
//!
//! 1. Create or use a collection of strategies.
//! 1. Create a struct which implement the `Supports` traits for each strategy.
//!    Each strategy has a corresponding `Supports` trait.
//!     - If you support all traits, the `SupportsAll`[^supports_all] trait will
//!       automatically be implemented.
//! 1. Create an instance of `Api`[^api], using your type.
//! 1. Pass this instance to any API function that requires it.
//!
//! # Strategies
//!
//! Here is a short description of each strategy:
//!
//! | strategy                                             | description                                      |
//! | :--------------------------------------------------- | :----------------------------------------------- |
//! | [crate::dev::strategy::alias::AliasStrategy]         | List and resolve aliases                         |
//! | [crate::dev::strategy::discovery::DiscoveryStrategy] | List and find repositories, independent of paths |
//! | [crate::dev::strategy::git::GitStrategy]             | Read and write operations using Git              |
//! | [crate::dev::strategy::path::PathStrategy]           | Resolve repositories to file system paths        |
//!
//! [^local_api]: [crate::dev::strategy::api::LocalApiStrategy]
//!
//! [^api]: [crate::dev::Api]
//!
//! [^supports_all]: [crate::dev::strategy::api::SupportsAll]

pub mod alias;
pub mod api;
pub mod discovery;
pub mod git;
pub mod path;
