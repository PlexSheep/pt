//! # Error module for [`pt-log`](crate)
//!
//! This module handles errors in logging contexts.

use anyhow;
use thiserror::Error;
use tracing::subscriber::SetGlobalDefaultError;
/// ## Errors for the [Logger](super::Logger)
#[derive(Error, Debug)]
pub enum Error {
    /// Bad IO operation
    #[error("Bad IO operation")]
    IO(#[from] std::io::Error),
    /// Various errors raised when the messenger is used in a wrong way
    #[error("Bad usage")]
    Usage(String),
    /// Could not assign logger as the global default
    #[error("Could not assign logger as global default")]
    SetGlobalDefaultFail(#[from] SetGlobalDefaultError),
    /// any other error type, wrapped in [`anyhow::Error`]
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
