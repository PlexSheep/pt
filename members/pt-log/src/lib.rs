//! # A specialized Logger for [`pt`](crate)
//!
//! For the library version, only the basic [`log`] is used, so that it is possible for
//! the end user to use the [`log`] frontend they desire.
//!
//! I did however decide to create a [`Logger`] struct. This struct is mainly intended to be used
//! with the python module of [`pt`](crate), but is still just as usable in other contexts.
//!
//! ## Technologies used for logging:
//! - [`log`]: base logging crate
//! - [`env_logger`]: used for the executable

//// ATTRIBUTES ////////////////////////////////////////////////////////////////////////////////////

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
use std::{
    fmt,
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
};

pub mod error;
use error::*;

pub use tracing::{debug, error, info, trace, warn, Level};
use tracing_appender;
use tracing_subscriber::{prelude::*, fmt::format::FmtSpan};
//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////
/// The log level used when none is specified
pub const DEFAULT_LOG_LEVEL: Level = Level::INFO;
/// The path where logs are stored when no path is given.
///
/// Currently, this is `/dev/null`, meaning they will be written to the void = discarded.
pub const DEFAULT_LOG_DIR: &'static str = "/dev/null";

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////
static INITIALIZED: AtomicBool = AtomicBool::new(false);

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////
/// ## Logger for [`pt`](crate)
///
/// This struct exists mainly for the python module, so that we can use the same logger with both
/// python and rust.
///
/// ### Setting a [`Level`](log::Level)
///
/// To set a [`Level`](log::Level), you need to set the environment variable `LIBPT_LOGLEVEL`
/// to either of:
///
/// - `Trace`
/// - `Debug`
/// - `Info`
/// - `Warn`
/// - `Error`
pub struct Logger {}

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////
/// ## Main implementation
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
    ///
    /// Assumes some defaults, use [`init_customized`](init_customized) for more control
    pub fn init(log_dir: Option<PathBuf>, max_level: Option<Level>) -> Result<()> {
        Self::init_customized(
            log_dir.is_some(),
            log_dir.unwrap_or(PathBuf::from(DEFAULT_LOG_DIR)),
            true,
            false,
            true,
            false,
            max_level.unwrap_or(DEFAULT_LOG_LEVEL),
            false,
            false,
            false,
        )
    }

    /// ## initializes the logger
    ///
    /// Will enable the logger to be used.
    pub fn init_customized(
        log_to_file: bool,
        log_dir: PathBuf,
        ansi: bool,
        display_filename: bool,
        display_level: bool,
        display_target: bool,
        max_level: Level,
        display_thread_ids: bool,
        display_thread_names: bool,
        display_line_number: bool,
    ) -> Result<()> {
        // only init if no init has been performed yet
        if INITIALIZED.load(Ordering::Relaxed) {
            warn!("trying to reinitialize the logger, ignoring");
            return Err(Error::Usage(format!("logging is already initialized")));
        } else {
            let filter = tracing_subscriber::filter::FilterFn::new(|metadata| {
                // let mut filter = false;
                //
                // // if it's this lib, continue
                // filter |= metadata.target().contains(env!("CARGO_PKG_NAME"));
                // filter |= metadata.target().contains("pt");
                //
                // // if it's another crate, only show above debug
                // filter |= metadata.level() > &Level::DEBUG;
                //
                // filter
                // FIXME: Make the filter customizable with sane defaults. Don't block the
                // executing crate.
                true
            });

            let basic_subscriber = tracing_subscriber::fmt::Subscriber::builder()
                // subscriber configuration
                .with_ansi(ansi)
                .with_file(display_filename)
                .with_level(display_level)
                .with_target(display_target)
                .with_max_level(max_level)
                .with_thread_ids(display_thread_ids)
                .with_line_number(display_line_number)
                .with_thread_names(display_thread_names)
                .with_span_events(FmtSpan::FULL)
                //.pretty // too verbose and over multiple lines, a bit like python tracebacks
                .finish()
                // add layers
                .with(filter);

            if log_to_file {
                let file_appender = tracing_appender::rolling::daily(log_dir, "log");
                let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
                let layered_subscriber = basic_subscriber
                    .with(tracing_subscriber::fmt::Layer::default().with_writer(file_writer));
                tracing::subscriber::set_global_default(layered_subscriber)?;
            } else {
                tracing::subscriber::set_global_default(basic_subscriber)?;
            }

            INITIALIZED.store(true, Ordering::Relaxed);
            Ok(())
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
