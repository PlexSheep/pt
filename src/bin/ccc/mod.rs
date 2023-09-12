//! # Executable for the math/compute submodule
//!
//! Calculate Calculations with your Computer!
//!
//! This command line tool allows you to input a mathematical expression. It will then process the
//! expression.

//// ATTRIBUTES ////////////////////////////////////////////////////////////////////////////////////
// we want docs
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
// we want Debug everywhere.
#![warn(missing_debug_implementations)]
// enable clippy's extra lints, the pedantic version
#![warn(clippy::pedantic)]

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
use pt::math::calculator::{*, self};
use pt::logger::*;

use clap::{Parser, Subcommand};
use clap_num::number_range;
use clap_verbosity_flag::{Verbosity, InfoLevel};

use std::path::PathBuf;

//// TYPES /////////////////////////////////////////////////////////////////////////////////////////

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////
/// short about section displayed in help
const ABOUT_ROOT: &'static str = r##"
Calculate Calculations with your Computer

    This commandline tool allows you to calculate complex mathematical expressions right in your
    shell.
"##;
/// longer about section displayed in help, is combined with [the short help](ABOUT_ROOT)
static LONG_ABOUT_ROOT: &'static str = r##"

    libpt is a personal general purpose library, offering this executable, a python module and a
    dynamic library.
"##;

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////
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

    /// your exporession(s)
    #[clap(trailing_var_arg=true)]
    pub expression: Vec<String>,
}

//// MACROS ////////////////////////////////////////////////////////////////////////////////////////

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////
fn main() {
    let cli = Cli::parse();
    let ll: tracing::Level = match cli.verbose.log_level().unwrap().as_str() {
        "TRACE" => tracing::Level::TRACE,
        "DEBUG" => tracing::Level::DEBUG,
        "INFO" => tracing::Level::INFO,
        "WARN" => tracing::Level::WARN,
        "ERROR" => tracing::Level::ERROR,
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
    let mut expr: String = String::new();
    for part in cli.expression {
        expr += &part;
    }

    debug!("exporssion: {}", expr);
    let r = Calculator::oneshot(expr);
    match r {
        Ok(r) => {
            println!("{r}");
        }
        Err(err) => {
            error!("Could not compute: {err}");
        }
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////