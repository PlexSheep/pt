//! # very short description
//!
//! Short description
//!
//! Details
//!
//! ## Section 1
//!
//! ## Section 2

//// ATTRIBUTES ////////////////////////////////////////////////////////////////////////////////////
// we want docs
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
// we want Debug everywhere.
#![warn(missing_debug_implementations)]
// enable clippy's extra lints, the pedantic version
#![warn(clippy::pedantic)]

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
use pyo3::{exceptions::PyException, PyErr};
use tracing::subscriber::SetGlobalDefaultError;

//// TYPES /////////////////////////////////////////////////////////////////////////////////////////
/// a quick alias for a result with a [`LoggerError`]
pub type Result<T> = std::result::Result<T, Error>;

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////

//// MACROS ////////////////////////////////////////////////////////////////////////////////////////

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////
/// ## Errors for the [logger](crate::logger)
pub enum Error {
    /// Bad IO operation
    IO(std::io::Error),
    /// Various errors raised when the messenger is used in a wrong way
    Usage(String),
    /// Could not assign logger as the global default
    SetGlobalDefaultFail(SetGlobalDefaultError),
}

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO(value)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<SetGlobalDefaultError> for Error {
    fn from(value: SetGlobalDefaultError) -> Self {
        Error::SetGlobalDefaultFail(value)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Into<PyErr> for Error {
    fn into(self) -> PyErr {
        match self {
            Error::IO(err) => PyException::new_err(format!("LoggerError: IO {err:?}")),
            Error::Usage(err) => PyException::new_err(format!("LoggerError: Usage {err}")),
            Error::SetGlobalDefaultFail(err) => {
                PyException::new_err(format!("LoggerError: SetGlobalDefaultFail {err}"))
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(e) => write!(f, "<IO Error {e:?}>"),
            Error::Usage(e) => write!(f, "<Usage Error {e:?}>"),
            Error::SetGlobalDefaultFail(e) => write!(f, "<SetGlobalDefaultFail {e:?}>"),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(e) => write!(f, "IO Error {e}"),
            Error::Usage(e) => write!(f, "Usage Error {e}"),
            Error::SetGlobalDefaultFail(e) => write!(f, "SetGlobalDefaultFail {e}"),
        }
    }
}

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////

