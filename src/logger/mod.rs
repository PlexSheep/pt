//! # A specialized Logger for [`libpt`](crate)
//!
//! For the library version, only the basic [`log`](crate::log) is used, so that it is possible for
//! the end user to use the [`log`](crate::log) frontend they desire.
//!
//! I did however decide to create a [`Logger`] struct. This struct is mainly intended to be used
//! with the python module of [`libpt`], but is still just as usable in other contexts.
//!
//! ## Technologies used for logging:
//! - [`log`](crate::log): base logging crate
//! - [`env_logger`](crate::env_logger): used for the executable

//// ATTRIBUTES ////////////////////////////////////////////////////////////////////////////////////

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
use std::{fmt, str::FromStr};

use log::{debug, error, info, trace, warn};

use env_logger;

use pyo3::prelude::*;
//// CONSTANTS ///////////////////////////////////////////////////////////////////////////////////////
/// The log level used when none is specified
const DEFAULT_LOG_LEVEL: log::Level = log::Level::Info;

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////
/// ## Logger for [`libpt`](crate::libpt)
///
/// This struct exists mainly for the python module, so that we can use the same logger with both
/// python and rust.
#[pyclass]
pub struct Logger {
    /// keeps track of if the logger was initialized
    pub initialized: bool,
}

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////
impl Logger {
    /// ## create a `Logger`
    pub fn new(level: log::Level) -> Self {
        let mut l = Logger { initialized: false };
        l.init(level);
        l
    }
    pub fn init(&mut self, level: log::Level) {
        // only init if no init has been performed yet
        if self.initialized {
            self.warn("trying to reinitialize the logger, ignoring");
            return;
        }
        #[allow(unused_imports)]
        use log::log_enabled;
        // TODO check if level is valid!
        std::env::set_var("RUST_LOG", level.as_str());
        env_logger::init();
        self.initialized = true;
    }

    /// ## logging at [`Level::Error`]
    pub fn error<T>(&self, printable: T)
    where
        T: fmt::Display,
    {
        error!("{}", printable)
    }
    /// ## logging at [`Level::Warn`]
    pub fn warn<T>(&self, printable: T)
    where
        T: fmt::Display,
    {
        warn!("{}", printable)
    }
    /// ## logging at [`Level::Info`]
    pub fn info<T>(&self, printable: T)
    where
        T: fmt::Display,
    {
        info!("{}", printable)
    }
    /// ## logging at [`Level::Debug`]
    pub fn debug<T>(&self, printable: T)
    where
        T: fmt::Display,
    {
        debug!("{}", printable)
    }
    /// ## logging at [`Level::Trace`]
    pub fn trace<T>(&self, printable: T)
    where
        T: fmt::Display,
    {
        trace!("{}", printable)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[pymethods]
impl Logger {
    /// ## Python version of [`new()`](Logger::new)
    #[new]
    pub fn py_new(level: String) -> PyResult<Self> {
        Ok(Self::new(log::Level::from_str(level.as_str()).expect(
            format!("could not get log level for '{}'", level).as_str(),
        )))
    }
    /// ## Python version of [`init()`](Logger::init)
    #[pyo3(name = "init")]
    pub fn py_init(&mut self, level: String) {
        Self::init(self, log::Level::from_str(level.as_str()).expect(
            format!("could not get log level for '{}'", level).as_str(),
        ))
    }
    /// ## Python version of [`error()`](Logger::error)
    #[pyo3(name = "error")]
    pub fn py_error(&self, printable: String) {
        self.error(printable)
    }
    /// ## Python version of [`warn()`](Logger::warn)
    #[pyo3(name = "warn")]
    pub fn py_warn(&self, printable: String) {
        self.warn(printable)
    }
    /// ## Python version of [`info()`](Logger::info)
    #[pyo3(name = "info")]
    pub fn py_info(&self, printable: String) {
        self.info(printable)
    }
    /// ## Python version of [`debug()`](Logger::debug)
    #[pyo3(name = "debug")]
    pub fn py_debug(&self, printable: String) {
        self.debug(printable)
    }
    /// ## Python version of [`trace()`](Logger::trace)
    #[pyo3(name = "trace")]
    pub fn py_trace(&self, printable: String) {
        self.trace(printable)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl fmt::Debug for Logger {
    /// ## Debug representation for [`Logger`]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Logger")
    }
}

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////
