//! Hermes: IBC Relayer CLI built in Rust
//!
//! The Hermes binary is a wrapper over the [ibc-relayer] library. This binary builds on
//! the [Abscissa] framework.
//!
//! For a comprehensive guide to using Hermes, the authoritative resource is
//! at [hermes.informal.systems].
//!
//! [ibc-relayer]: https://docs.rs/ibc-relayer/0.2.0/
//! [Abscissa]: https://github.com/iqlusioninc/abscissa
//! [hermes.informal.systems]: https://hermes.informal.systems

// Tip: Deny warnings with `RUSTFLAGS="-D warnings"` environment variable in CI

#![forbid(unsafe_code)]
#![deny(
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]

pub mod application;
pub mod commands;
pub mod config;
pub mod error;
pub mod prelude;
pub mod registry;

pub(crate) mod cli_utils;
pub(crate) mod components;
pub(crate) mod conclude;
pub(crate) mod entry;

/// The path to the default configuration file.
pub const DEFAULT_CONFIG_PATH: &str = ".hermes/config.toml";
