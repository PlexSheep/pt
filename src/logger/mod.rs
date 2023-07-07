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
use std::{
    fmt,
    io::Write,
    sync::atomic::{AtomicBool, Ordering},
};

use env_logger::{
    fmt::{Formatter, Style},
    Env, Target, WriteStyle,
};
use log::{debug, error, info, trace, warn, Level};

use pyo3::prelude::*;
//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////
/// The log level used when none is specified
pub const DEFAULT_LOG_LEVEL: Level = Level::Info;
/// Register your level to this ENVAR to override the used level
pub const LOGGER_ENV_KEY: &'static str = "LIBPT_LOGLEVEL";

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////
static INITIALIZED: AtomicBool = AtomicBool::new(false);

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////
/// ## Logger for [`libpt`](crate::libpt)
///
/// This struct exists mainly for the python module, so that we can use the same logger with both
/// python and rust.
///
/// ### Setting a [`Level`](log::Level)
///
/// To set a [`Level`](log::Level), you need to set the ENVAR `LIBPT_LOGLEVEL` to either of
///
/// - `Trace`
/// - `Debug`
/// - `Info`
/// - `Warn`
/// - `Error`
#[pyclass]
pub struct Logger {}

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////
impl Logger {
    /// ## create a `Logger`
    ///
    /// Creates a new uninitialized [`Logger`] object.
    pub fn new() -> Self {
        let l = Logger {};
        l
    }
    /// ## initializes the logger
    ///
    /// Will enable the logger to be used.
    pub fn init() {
        // only init if no init has been performed yet
        if INITIALIZED.load(Ordering::Relaxed) {
            warn!("trying to reinitialize the logger, ignoring");
            return;
        } else {
            let env = Env::default().filter_or(LOGGER_ENV_KEY, DEFAULT_LOG_LEVEL.to_string());
            env_logger::init_from_env(env);
            INITIALIZED.store(true, Ordering::Relaxed);
        }
    }

    /// ## initializes the logger to log to a target
    ///
    /// Will enable the logger to be used.
    pub fn init_specialized(test: bool, color: bool, target: Target) {
        // only init if no init has been performed yet
        if INITIALIZED.load(Ordering::Relaxed) {
            warn!("trying to reinitialize the logger, ignoring");
            return;
        } else {
            let env = Env::default().filter_or(LOGGER_ENV_KEY, DEFAULT_LOG_LEVEL.to_string());
            env_logger::Builder::from_env(env)
                .is_test(test)
                .target(target)
                .write_style(if color {
                    WriteStyle::Auto
                } else {
                    WriteStyle::Never
                })
                .init();
            INITIALIZED.store(true, Ordering::Relaxed);
        }
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
    pub fn py_new() -> PyResult<Self> {
        Ok(Logger::new())
    }
    /// ## Python version of [`init()`](Logger::init)
    #[pyo3(name = "init")]
    #[staticmethod]
    pub fn py_init() {
        Self::init()
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
        write!(
            f,
            "Logger: {{initialized: {}}} ",
            INITIALIZED.load(Ordering::Relaxed)
        )
    }
}

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////
