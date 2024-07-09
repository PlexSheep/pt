//! Utilities for parsing options and arguments on the start of a CLI application

use clap::Parser;
use libpt_log::Level;
#[cfg(feature = "log")]
use log;

/// Custom help template for displaying command-line usage information
///
/// This template modifies the default template provided by Clap to include additional information
/// and customize the layout of the help message.
///
/// Differences from the default template:
/// - Includes the application version and author information at the end
///
/// Apply like this:
/// ```
/// # use libpt_cli::args::HELP_TEMPLATE;
/// use clap::Parser;
/// #[derive(Parser, Debug, Clone, PartialEq, Eq, Hash)]
/// #[command(help_template = HELP_TEMPLATE)]
/// pub struct MyArgs {
///     /// show more details
///     #[arg(short, long)]
///     pub verbose: bool,
/// }
/// ```
///
/// ## Example
///
/// Don't forget to set `authors` in your `Cargo.toml`!
///
/// ```bash
/// $ cargo run -- -h
/// about: short
///
/// Usage: aaa [OPTIONS]
///
/// Options:
///   -v, --verbose  show more details
///   -h, --help     Print help (see more with '--help')
///   -V, --version  Print version
///
/// aaa: 0.1.0
/// Author: Christoph J. Scherr <software@cscherr.de>
///
/// ```
pub const HELP_TEMPLATE: &str = r"{about-section}
{usage-heading} {usage}

{all-args}{tab}

{name}: {version}
Author: {author-with-newline}
";

/// Transform -v and -q flags to some kind of loglevel
///
/// # Example
///
/// Include this into your [clap] derive struct like this:
///
/// ```
/// use libpt_cli::args::VerbosityLevel;
/// use clap::Parser;
///
/// #[derive(Parser, Debug)]
/// pub struct Opts {
///     #[command(flatten)]
///     pub verbose: VerbosityLevel,
///     #[arg(short, long)]
///     pub mynum: usize,
/// }
///
/// ```
///
/// Get the loglevel like this:
///
/// ```no_run
/// # use libpt_cli::args::VerbosityLevel;
/// use libpt_log::Level;
/// # use clap::Parser;
///
/// # #[derive(Parser, Debug)]
/// # pub struct Opts {
/// #     #[command(flatten)]
/// #     pub verbose: VerbosityLevel,
/// # }
///
/// fn main() {
///     let opts = Opts::parse();
///
///     // Level might be None if the user wants no output at all.
///     // for the 'tracing' level:
///     let level: Level = opts.verbose.level();
/// }
/// ```
#[derive(Parser, Clone, PartialEq, Eq, Hash)]
pub struct VerbosityLevel {
    /// make the output more verbose
    #[arg(
        long,
        short = 'v',
        action = clap::ArgAction::Count, // NOTE: this forces u8 type for some reason
        global = true,
        // help = L::verbose_help(),
        // long_help = L::verbose_long_help(),
    )]
    verbose: u8,

    /// make the output less verbose
    ///
    /// ( -qqq for completely quiet)
    #[arg(
        long,
        short = 'q',
        action = clap::ArgAction::Count,
        global = true,
        conflicts_with = "verbose",
    )]
    quiet: u8,
}

impl VerbosityLevel {
    /// true only if no verbose and no quiet was set (user is using defaults)
    #[inline]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // the values of self can change
    pub fn changed(&self) -> bool {
        self.verbose != 0 || self.quiet != 0
    }
    #[inline]
    #[must_use]
    fn value(&self) -> u8 {
        Self::level_value(Level::INFO)
            .saturating_sub((self.quiet).min(10))
            .saturating_add((self.verbose).min(10))
    }

    /// get the [Level] for that [`VerbosityLevel`]
    ///
    /// # Examples
    ///
    /// ```
    /// use libpt_log::Level; // reexport: tracing
    /// use libpt_cli::args::VerbosityLevel;
    ///
    /// let verbosity_level = VerbosityLevel::INFO;
    /// assert_eq!(verbosity_level.level(), Level::INFO);
    /// ```
    #[inline]
    #[must_use]
    pub fn level(&self) -> Level {
        let v = self.value();
        match v {
            0 => Level::ERROR,
            1 => Level::WARN,
            2 => Level::INFO,
            3 => Level::DEBUG,
            4 => Level::TRACE,
            _ => {
                if v > 4 {
                    Level::TRACE
                } else {
                    /* v < 0 */
                    Level::ERROR
                }
            }
        }
    }

    /// get the [`log::Level`] for that `VerbosityLevel`
    ///
    /// This is the method for the [log] crate, which I use less often.
    ///
    /// [None] means that absolutely no output is wanted (completely quiet)
    #[inline]
    #[must_use]
    #[cfg(feature = "log")]
    pub fn level_for_log_crate(&self) -> log::Level {
        match self.level() {
            Level::TRACE => log::Level::Trace,
            Level::DEBUG => log::Level::Debug,
            Level::INFO => log::Level::Info,
            Level::WARN => log::Level::Warn,
            Level::ERROR => log::Level::Error,
        }
    }

    #[inline]
    #[must_use]
    const fn level_value(level: Level) -> u8 {
        match level {
            Level::TRACE => 4,
            Level::DEBUG => 3,
            Level::INFO => 2,
            Level::WARN => 1,
            Level::ERROR => 0,
        }
    }

    /// # Examples
    ///
    /// ```
    /// use libpt_log::Level; // reexport: tracing
    /// use libpt_cli::args::VerbosityLevel;
    ///
    /// let verbosity_level = VerbosityLevel::TRACE;
    /// assert_eq!(verbosity_level.level(), Level::TRACE);
    /// ```
    pub const TRACE: Self = Self {
        verbose: 2,
        quiet: 0,
    };
    /// # Examples
    ///
    /// ```
    /// use libpt_log::Level; // reexport: tracing
    /// use libpt_cli::args::VerbosityLevel;
    ///
    /// let verbosity_level = VerbosityLevel::DEBUG;
    /// assert_eq!(verbosity_level.level(), Level::DEBUG);
    /// ```
    pub const DEBUG: Self = Self {
        verbose: 1,
        quiet: 0,
    };
    /// # Examples
    ///
    /// ```
    /// use libpt_log::Level; // reexport: tracing
    /// use libpt_cli::args::VerbosityLevel;
    ///
    /// let verbosity_level = VerbosityLevel::INFO;
    /// assert_eq!(verbosity_level.level(), Level::INFO);
    /// ```
    pub const INFO: Self = Self {
        verbose: 0,
        quiet: 0,
    };
    /// # Examples
    ///
    /// ```
    /// use libpt_log::Level; // reexport: tracing
    /// use libpt_cli::args::VerbosityLevel;
    ///
    /// let verbosity_level = VerbosityLevel::WARN;
    /// assert_eq!(verbosity_level.level(), Level::WARN);
    /// ```
    pub const WARN: Self = Self {
        verbose: 0,
        quiet: 1,
    };
    /// # Examples
    ///
    /// ```
    /// use libpt_log::Level; // reexport: tracing
    /// use libpt_cli::args::VerbosityLevel;
    ///
    /// let verbosity_level = VerbosityLevel::ERROR;
    /// assert_eq!(verbosity_level.level(), Level::ERROR);
    /// ```
    pub const ERROR: Self = Self {
        verbose: 0,
        quiet: 2,
    };
}

impl std::fmt::Debug for VerbosityLevel {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.level())
    }
}

impl Default for VerbosityLevel {
    fn default() -> Self {
        Self::INFO
    }
}
