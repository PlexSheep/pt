use clap::Parser;
use libpt_log::Level;

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
pub const HELP_TEMPLATE: &str = r#"{about-section}
{usage-heading} {usage}

{all-args}{tab}

{name}: {version}
Author: {author-with-newline}
"#;

#[derive(Parser, Debug, Clone, PartialEq, Eq, Hash)]
#[command(help_template = HELP_TEMPLATE)]
pub struct DefaultArguments {
    /// get a [tracing] log level
    ///
    /// set the verbosity with repeated '-q' and '-v' flags
    #[command(flatten)]
    verbose: VerbosityLevel,
}

#[derive(Parser, Clone, PartialEq, Eq, Hash)]
pub struct VerbosityLevel {
    /// make the output more verbose
    #[arg(
        long,
        short = 'v',
        action = clap::ArgAction::Count,
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
    pub fn changed(&self) -> bool {
        self.verbose != 0 || self.quiet != 0
    }
    #[inline]
    fn value(&self) -> i8 {
        let v = Self::level_value(Level::INFO) - (self.quiet as i8) + (self.verbose as i8);
        if v > Self::level_value(Level::TRACE) {
            Self::level_value(Level::TRACE)
        } else {
            v
        }
    }

    /// get the [Level] for that VerbosityLevel
    ///
    /// [None] means that absolutely no output is wanted (completely quiet)
    #[inline]
    pub fn level(&self) -> Option<Level> {
        Some(match self.value() {
            0 => Level::ERROR,
            1 => Level::WARN,
            2 => Level::INFO,
            3 => Level::DEBUG,
            4 => Level::TRACE,
            _ => return None,
        })
    }
    #[inline]
    fn level_value(level: Level) -> i8 {
        match level {
            Level::TRACE => 4,
            Level::DEBUG => 3,
            Level::INFO => 2,
            Level::WARN => 1,
            Level::ERROR => 0,
        }
    }
}

impl std::fmt::Debug for VerbosityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.level())
    }
}
