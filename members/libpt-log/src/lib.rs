//! # A specialized Logger for [`pt`](../libpt/index.html)
//!
//! This crate is part of [`pt`](../libpt/index.html), but can also be used as a standalone
//! module.
//!
//! For the library version, only the basic [`tracing`] is used, so that it is possible for
//! the end user to use the [`tracing`] frontend they desire.
//!
//! I did however decide to create a [`Logger`] struct. This struct is mainly intended to be used
//! with the python module of [`pt`](../libpt/index.html), but is still just as usable in other contexts.
//!
//! ## Technologies used for logging:
//! - [`tracing`]: base logging crate
//! - [`tracing_appender`]: Used to log to files
//! - [`tracing_subscriber`]: Used to do actual logging, formatting, to stdout

use std::{
    fmt,
    path::PathBuf,
    sync::atomic::{AtomicBool, Ordering},
};

pub mod error;
use error::*;

pub use tracing;
pub use tracing::{debug, error, info, trace, warn, Level};
use tracing_appender::{self, non_blocking::NonBlocking};
use tracing_subscriber::fmt::{format::FmtSpan, time};

use anyhow::{bail, Result};
/// The log level used when none is specified
pub const DEFAULT_LOG_LEVEL: Level = Level::INFO;
/// The path where logs are stored when no path is given.
///
/// Currently, this is `/dev/null`, meaning they will be written to the void = discarded.
pub const DEFAULT_LOG_DIR: &str = "/dev/null";

static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Builder for a well configured [Logger]
///
/// This struct helps configure a global logger that can be used with either macros or methods, see
/// [Logger].
///
/// ## Examples
///
/// ```
/// # use libpt_log::{Logger, info};
/// # fn main() {
/// Logger::builder()
///     .uptime(true)
///     .build();
/// info!("hello world");
/// # }
///
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct LoggerBuilder {
    /// create and log to logfiles
    log_to_file: bool,
    /// logfiles would be created here
    log_dir: PathBuf,
    /// use ANSI control sequences
    ansi: bool,
    /// show which source file produces a log
    display_filename: bool,
    /// show the log level of the message
    display_level: bool,
    /// show target context
    display_target: bool,
    /// sets the maximum verbosity level.
    ///
    /// For example, if set to [Error](Level::ERROR), logs at [Info](Level::INFO) will not be
    /// printed. If set to [Debug](Level::DEBUG), logs at [Info](Level::INFO) will be printed.
    max_level: Level,
    /// show the id of the thread that created this message
    display_thread_ids: bool,
    /// show the name of the thread that created this message
    display_thread_names: bool,
    /// show which line in the source file produces a log
    display_line_number: bool,
    /// splits a log over multiple lines, looks like a python traceback
    pretty: bool,
    /// show when the log was created
    show_time: bool,
    /// show timestamps as uptime (duration since the logger was initialized)
    uptime: bool,
}

impl LoggerBuilder {
    /// use the configured settings to build and initialize a new global [Logger]
    ///
    /// This will build a functional [Logger]. You don't need to use the [Logger] struct, it's
    /// better to use the macros:
    ///
    /// * `error!`
    /// * `warn!`
    /// * `info!`
    /// * `debug!`
    /// * `trace!`
    ///
    /// instead of the methods of the [Logger] struct. You can however use the [Logger] struct in
    /// cases where usage of a macro is bad or you are somehow working with multiple loggers.
    ///
    /// ## Examples
    ///
    /// ```
    /// # use libpt_log::{Logger, info};
    /// # fn main() {
    /// Logger::builder()
    ///     .uptime(true)
    ///     .build();
    /// info!("hello world");
    /// # }
    ///
    /// ```
    /// # Errors
    ///
    /// This function will return an error if a global Logger was aready initialized. This module
    /// uses the [tracing] crate for logging, so if a [tracing] logger is initialized elsewhere,
    /// this method will error.
    pub fn build(self) -> Result<Logger> {
        // only init if no init has been performed yet
        if INITIALIZED.load(Ordering::Relaxed) {
            warn!("trying to reinitialize the logger, ignoring");
            bail!(Error::Usage("logging is already initialized".to_string()));
        }
        let subscriber = tracing_subscriber::fmt::Subscriber::builder()
            .with_level(self.display_level)
            .with_max_level(self.max_level)
            .with_ansi(self.ansi)
            .with_target(self.display_target)
            .with_file(self.display_filename)
            .with_thread_ids(self.display_thread_ids)
            .with_line_number(self.display_line_number)
            .with_thread_names(self.display_thread_names)
            .with_span_events(FmtSpan::FULL);
        // I know this is hacky, but I couldn't get it any other way. I couldn't even find a
        // project that could do it any other way. You can't apply one after another, because the
        // type is changed every time. When using `Box<dyn Whatever>`, some methods complain about
        // not being in trait bounds.
        // TODO: somehow find a better solution for this
        match (self.log_to_file, self.show_time, self.pretty, self.uptime) {
            (true, true, true, true) => {
                let subscriber = subscriber
                    .with_writer(new_file_appender(self.log_dir))
                    .with_timer(time::uptime())
                    .pretty()
                    .finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (true, true, true, false) => {
                let subscriber = subscriber
                    .with_writer(new_file_appender(self.log_dir))
                    .pretty()
                    .finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (true, false, true, _) => {
                let subscriber = subscriber
                    .with_writer(new_file_appender(self.log_dir))
                    .without_time()
                    .pretty()
                    .finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (true, true, false, true) => {
                let subscriber = subscriber
                    .with_writer(new_file_appender(self.log_dir))
                    .with_timer(time::uptime())
                    .finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (true, true, false, false) => {
                let subscriber = subscriber
                    .with_writer(new_file_appender(self.log_dir))
                    .finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (true, false, false, _) => {
                let file_appender = tracing_appender::rolling::daily(self.log_dir.clone(), "log");
                let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
                let subscriber = subscriber.with_writer(file_writer).without_time().finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (false, true, true, true) => {
                let subscriber = subscriber.pretty().with_timer(time::uptime()).finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (false, true, true, false) => {
                let subscriber = subscriber.pretty().with_timer(time::uptime()).finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (false, false, true, _) => {
                let subscriber = subscriber.without_time().pretty().finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (false, true, false, true) => {
                let subscriber = subscriber.with_timer(time::uptime()).finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (false, true, false, false) => {
                let subscriber = subscriber.finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (false, false, false, _) => {
                let subscriber = subscriber.without_time().finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
        }
        INITIALIZED.store(true, Ordering::Relaxed);
        Ok(Logger {})
    }

    /// enable or disable logging to and creating of logfiles
    pub fn log_to_file(mut self, log_to_file: bool) -> Self {
        self.log_to_file = log_to_file;
        self
    }

    /// set a directory where logfiles would be created in
    ///
    /// Enable or disable creation and logging to logfiles with [log_to_file](Self::log_to_file).
    ///
    /// The default logdir is [DEFAULT_LOG_DIR].
    pub fn log_dir(mut self, log_dir: PathBuf) -> Self {
        self.log_dir = log_dir;
        self
    }

    /// enable or disable ANSI control sequences
    ///
    /// Disabling ANSI control sequences might improve compatibility and readability when the logs
    /// are displayed by a program that does not interpret them.
    ///
    /// Keeping ANSI control sequences enabled has the disadvantage of added colors for the logs.
    pub fn ansi(mut self, ansi: bool) -> Self {
        self.ansi = ansi;
        self
    }

    /// when making a log, display the source file in which a log was crated in
    pub fn display_filename(mut self, display_filename: bool) -> Self {
        self.display_filename = display_filename;
        self
    }

    /// when making a log, display the log level of the message
    pub fn display_level(mut self, display_level: bool) -> Self {
        self.display_level = display_level;
        self
    }

    /// show target context
    pub fn display_target(mut self, display_target: bool) -> Self {
        self.display_target = display_target;
        self
    }

    /// set the maximum verbosity level.
    pub fn max_level(mut self, max_level: Level) -> Self {
        self.max_level = max_level;
        self
    }

    /// show the id of the thread that created this message
    pub fn display_thread_ids(mut self, display_thread_ids: bool) -> Self {
        self.display_thread_ids = display_thread_ids;
        self
    }

    /// show the name of the thread that created this message
    pub fn display_thread_names(mut self, display_thread_names: bool) -> Self {
        self.display_thread_names = display_thread_names;
        self
    }

    /// show which line in the source file produces a log
    pub fn display_line_number(mut self, display_line_number: bool) -> Self {
        self.display_line_number = display_line_number;
        self
    }

    /// splits a log over multiple lines, looks like a python traceback
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = pretty;
        self
    }

    /// show a timestamp describing when the log was created
    pub fn show_time(mut self, show_time: bool) -> Self {
        self.show_time = show_time;
        self
    }

    /// show timestamps as uptime (duration since the logger was initialized)
    pub fn uptime(mut self, uptime: bool) -> Self {
        self.uptime = uptime;
        self
    }
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        Self {
            log_to_file: false,
            log_dir: PathBuf::from(DEFAULT_LOG_DIR),
            ansi: true,
            display_filename: false,
            display_level: true,
            display_target: false,
            max_level: DEFAULT_LOG_LEVEL,
            display_thread_ids: false,
            display_thread_names: false,
            display_line_number: false,
            pretty: false,
            show_time: true,
            uptime: false,
        }
    }
}

/// ## Logger for [`pt`](libpt)
///
/// A logger is generally a functionality that let's you write information from your library or
/// application in a more structured manner than if you just wrote all information to `stdout` or
/// `stderr` with the likes of `println!` or `eprintln!`.
///
/// It offers writing to multiple targets, such as both the terminal and a log file, and allows
/// users to choose the verbosity of the information that gets printed by selecting a
/// [Loglevel](Level).
///
/// ## Levels
///
/// TODO: add levels desc and ascii art
///
/// ## Usage
///
/// You don't need to use the [Logger] struct, it's better to use the macros instead:
///
/// * `error!`
/// * `warn!`
/// * `info!`
/// * `debug!`
/// * `trace!`
///
/// You can however use the [Logger] struct in cases where usage of a macro is bad or
/// you are somehow working with multiple loggers. The macros offer additional functionalities,
/// suck as full `format!` support and context, see [`tracing`], which we use as backend.
///
/// ## Examples
///
/// ```
/// # use libpt_log::{Logger, info};
/// # fn main() {
/// Logger::builder()
///     .uptime(true)
///     .build();
/// info!("hello world");
/// # }
///
/// ```
pub struct Logger;

/// ## Main implementation
impl Logger {
    /// Get a new [LoggerBuilder]
    pub fn builder() -> LoggerBuilder {
        LoggerBuilder::default()
    }

    /// ## initializes the logger
    ///
    /// Will enable the logger to be used.
    ///
    /// Assumes some defaults, use [`init_customized`](Self::init_customized) for more control
    #[deprecated(since = "0.4.1", note = "use Logger::builder() instead")]
    pub fn build(log_dir: Option<PathBuf>, max_level: Option<Level>, uptime: bool) -> Result<Self> {
        #[allow(deprecated)]
        Self::build_customized(
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
            false,
            true,
            uptime,
        )
    }

    /// ## initializes the logger
    ///
    /// Will enable the logger to be used. This is a version that shows less information,
    /// useful in cases with only one sender to the logging framework.
    ///
    /// Assumes some defaults, use [`init_customized`](Self::init_customized) for more control
    #[deprecated(since = "0.4.1", note = "use Logger::builder() instead")]
    pub fn build_mini(max_level: Option<Level>) -> Result<Self> {
        #[allow(deprecated)]
        Self::build_customized(
            false,
            PathBuf::from(DEFAULT_LOG_DIR),
            true,
            false,
            true,
            false,
            max_level.unwrap_or(DEFAULT_LOG_LEVEL),
            false,
            false,
            false,
            false,
            false,
            false,
        )
    }

    /// ## initializes the logger
    ///
    /// Will enable the logger to be used.
    #[deprecated(since = "0.4.1", note = "use Logger::builder() instead")]
    #[allow(clippy::too_many_arguments)]
    pub fn build_customized(
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
        pretty: bool,
        show_time: bool,
        uptime: bool, // uptime instead of system time
    ) -> Result<Self> {
        // only init if no init has been performed yet
        if INITIALIZED.load(Ordering::Relaxed) {
            warn!("trying to reinitialize the logger, ignoring");
            bail!(Error::Usage("logging is already initialized".to_string()));
        }
        let subscriber = tracing_subscriber::fmt::Subscriber::builder()
            .with_level(display_level)
            .with_max_level(max_level)
            .with_ansi(ansi)
            .with_target(display_target)
            .with_file(display_filename)
            .with_thread_ids(display_thread_ids)
            .with_line_number(display_line_number)
            .with_thread_names(display_thread_names)
            .with_span_events(FmtSpan::FULL);
        // I know this is hacky, but I couldn't get it any other way. I couldn't even find a
        // project that could do it any other way. You can't apply one after another, because the
        // type is changed every time. When using Box<dyn Whatever>, some methods complain about
        // not being in trait bounds.
        // TODO: somehow find a better solution for this
        match (log_to_file, show_time, pretty, uptime) {
            (true, true, true, true) => {
                let subscriber = subscriber
                    .with_writer(new_file_appender(log_dir))
                    .with_timer(time::uptime())
                    .pretty()
                    .finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (true, true, true, false) => {
                let subscriber = subscriber
                    .with_writer(new_file_appender(log_dir))
                    .pretty()
                    .finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (true, false, true, _) => {
                let subscriber = subscriber
                    .with_writer(new_file_appender(log_dir))
                    .without_time()
                    .pretty()
                    .finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (true, true, false, true) => {
                let subscriber = subscriber
                    .with_writer(new_file_appender(log_dir))
                    .with_timer(time::uptime())
                    .finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (true, true, false, false) => {
                let subscriber = subscriber.with_writer(new_file_appender(log_dir)).finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (true, false, false, _) => {
                let file_appender = tracing_appender::rolling::daily(log_dir.clone(), "log");
                let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
                let subscriber = subscriber.with_writer(file_writer).without_time().finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (false, true, true, true) => {
                let subscriber = subscriber.pretty().with_timer(time::uptime()).finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (false, true, true, false) => {
                let subscriber = subscriber.pretty().with_timer(time::uptime()).finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (false, false, true, _) => {
                let subscriber = subscriber.without_time().pretty().finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (false, true, false, true) => {
                let subscriber = subscriber.with_timer(time::uptime()).finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (false, true, false, false) => {
                let subscriber = subscriber.finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
            (false, false, false, _) => {
                let subscriber = subscriber.without_time().finish();
                tracing::subscriber::set_global_default(subscriber)?;
            }
        }
        INITIALIZED.store(true, Ordering::Relaxed);
        Ok(Logger {})
    }

    /// ## logging at [`Level::ERROR`]
    pub fn error<T>(&self, printable: T)
    where
        T: fmt::Display,
    {
        error!("{}", printable)
    }
    /// ## logging at [`Level::WARN`]
    pub fn warn<T>(&self, printable: T)
    where
        T: fmt::Display,
    {
        warn!("{}", printable)
    }
    /// ## logging at [`Level::INFO`]
    pub fn info<T>(&self, printable: T)
    where
        T: fmt::Display,
    {
        info!("{}", printable)
    }
    /// ## logging at [`Level::DEBUG`]
    pub fn debug<T>(&self, printable: T)
    where
        T: fmt::Display,
    {
        debug!("{}", printable)
    }
    /// ## logging at [`Level::TRACE`]
    pub fn trace<T>(&self, printable: T)
    where
        T: fmt::Display,
    {
        trace!("{}", printable)
    }
}

impl fmt::Debug for Logger {
    /// ## DEBUG representation for [`Logger`]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Logger: {{initialized: {}}} ",
            INITIALIZED.load(Ordering::Relaxed)
        )
    }
}

impl Default for Logger {
    fn default() -> Self {
        LoggerBuilder::default()
            .build()
            .expect("building a Logger failed")
    }
}

fn new_file_appender(log_dir: PathBuf) -> NonBlocking {
    let file_appender = tracing_appender::rolling::daily(log_dir.clone(), "log");
    tracing_appender::non_blocking(file_appender).0
}
