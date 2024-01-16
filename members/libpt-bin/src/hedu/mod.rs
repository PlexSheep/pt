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

use std::{fs::File, io::BufReader, path::PathBuf};

//// TYPES /////////////////////////////////////////////////////////////////////////////////////////

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////
/// short about section displayed in help
const ABOUT_ROOT: &'static str = r##"
Dumps data in fancy formats
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
r#"libpt: {version}{about-section}Author:
{author-with-newline}
{usage-heading} {usage}{all-args}{tab}"#
    )]
pub struct Cli {
    // clap_verbosity_flag seems to make this a global option implicitly
    /// set a verbosity, multiple allowed (f.e. -vvv)
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    /// show logger meta
    #[arg(short, long, global = true)]
    pub log_meta: bool,

    /// show character representation
    #[arg(short, long, global = true)]
    pub chars: bool,

    /// a data source, probably a file
    pub data_source: String,
}

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////
fn main() {
    let cli = cli_parse();
    debug!("Trying to open '{}'", cli.data_source);
    let file = match File::open(cli.data_source.clone()) {
        Ok(file) => file,
        Err(err) => {
            error!("Could not open file '{}': {err}", cli.data_source);
            std::process::exit(1);
        }
    };
    match dump(BufReader::new(file), cli.chars) {
        Ok(_) => (),
        Err(err) => {
            error!("Could not dump data of file: {err}");
            std::process::exit(2);
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
            eprintln!("'{}' is not a valid loglevel", cli.verbose.to_string());
            std::process::exit(1);
        }
    };
    if cli.log_meta {
        Logger::init_customized(
            false,
            PathBuf::from("/dev/null"),
            true,
            false,
            true,
            true,
            ll,
            false,
            false,
            false,
        )
        .expect("could not initialize Logger");
    } else {
        // less verbose version
        Logger::init_customized(
            false,
            PathBuf::from("/dev/null"),
            true,
            false,
            true,
            false,
            ll,
            false,
            false,
            false,
        )
        .expect("could not initialize Logger");
    }
    return cli;
}
