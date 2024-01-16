//! # Executable for the hedu submodule
//!
//! Dump data to a fancy format.

//// ATTRIBUTES ////////////////////////////////////////////////////////////////////////////////////
// we want docs
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
// we want Debug everywhere.
#![warn(missing_debug_implementations)]
// enable clippy's extra lints, the pedantic version
#![warn(clippy::pedantic)]

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////

use libpt::{hedu::*, log::*};

use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

use std::{
    fs::File,
    io::{Seek, IsTerminal},
};

//// TYPES /////////////////////////////////////////////////////////////////////////////////////////

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////
/// short about section displayed in help
const ABOUT_ROOT: &'static str = r##"
Dumps data in fancy formats.
"##;
/// longer about section displayed in help, is combined with [the short help](ABOUT_ROOT)
static LONG_ABOUT_ROOT: &'static str = r##"

    libpt is a personal general purpose library, offering this executable, a python module and a
    dynamic library.
"##;

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////

//// MACROS ////////////////////////////////////////////////////////////////////////////////////////

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////
/// defines CLI interface
#[derive(Debug, Clone, Parser)]
#[command(
    author,
    version,
    about = ABOUT_ROOT,
    long_about = format!("{}{}", ABOUT_ROOT ,LONG_ABOUT_ROOT),
    help_template =
r#"{about-section}
{usage-heading} {usage}
{all-args}{tab}

libpt: {version}
Author: {author-with-newline}
"#
    )]
pub struct Cli {
    // clap_verbosity_flag seems to make this a global option implicitly
    /// set a verbosity, multiple allowed (f.e. -vvv)
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    /// show additional logging meta data
    #[arg(long)]
    pub meta: bool,

    /// show character representation
    #[arg(short, long)]
    pub chars: bool,

    /// skip first N bytes
    #[arg(short, long, default_value_t = 0)]
    pub skip: usize,

    /// only interpret N bytes (end after N)
    #[arg(short, long, default_value_t = 0)]
    pub len: usize,

    /// show identical lines
    #[arg(short = 'i', long)]
    pub show_identical: bool,

    /// a data source, probably a file.
    ///
    /// If left empty or set as "-", the program will read from stdin.
    pub data_source: Option<String>,
}

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////
fn main() {
    let cli = cli_parse();
    let mut source: Box<dyn DataSource>;
    if cli.data_source.is_some() && cli.data_source.clone().is_some_and(|val| val != "-") {
        let data_source = cli.data_source.unwrap();
        debug!("Trying to open '{}'", data_source);
        source = match File::open(&data_source) {
            Ok(file) => Box::new(file),
            Err(err) => {
                error!("Could not open file '{}': {err}", data_source);
                std::process::exit(1);
            }
        };
    } else {
        debug!("Trying to open stdout");
        let stdin = std::io::stdin();
        if stdin.is_terminal() {
            warn!("Refusing to dump from interactive terminal");
            std::process::exit(2)
        }
        source = Box::new(stdin);
    }

    match dump(
        &mut *source,
        DumpConfig {
            chars: cli.chars,
            skip: cli.skip,
            show_identical: cli.show_identical,
            len: cli.len,
        },
    ) {
        Ok(_) => (),
        Err(err) => {
            error!("Could not dump data of file: {err}");
            std::process::exit(3);
        }
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
fn cli_parse() -> Cli {
    let cli = Cli::parse();
    let ll: Level = match cli.verbose.log_level().unwrap().as_str() {
        "TRACE" => Level::TRACE,
        "DEBUG" => Level::DEBUG,
        "INFO" => Level::INFO,
        "WARN" => Level::WARN,
        "ERROR" => Level::ERROR,
        _ => {
            unreachable!();
        }
    };
    if cli.meta {
        Logger::init(None, Some(ll)).expect("could not initialize Logger");
    } else {
        // less verbose version
        Logger::init_mini(Some(ll)).expect("could not initialize Logger");
    }
    return cli;
}
