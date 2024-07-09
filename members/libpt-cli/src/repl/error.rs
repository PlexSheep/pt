//! Errors for the Repl module

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parsing(#[from] clap::Error),
    #[error(transparent)]
    Input(#[from] dialoguer::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
