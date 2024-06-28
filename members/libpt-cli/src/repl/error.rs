use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReplError {
    #[error(transparent)]
    Parsing(#[from] clap::Error),
    #[error(transparent)]
    Input(#[from] dialoguer::Error),
}
