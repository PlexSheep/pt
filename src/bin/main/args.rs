//! # args module
//!
//! The args module of pt is used to parse commandline arguments. For this, it makes use of
//! [`clap`].

//// ATTRIBUTES ////////////////////////////////////////////////////////////////////////////////////
// we want docs
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
////////////////////////////////////////////////////////////////////////////////////////////////////
// we want Debug everywhere.
#![warn(missing_debug_implementations)]
////////////////////////////////////////////////////////////////////////////////////////////////////
// enable clippy's extra lints, the pedantic version
#![warn(clippy::pedantic)]

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
use clap::{Parser, Subcommand};

use clap_num::number_range;

use clap_verbosity_flag::{Verbosity, InfoLevel};

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////
/// short about section displayed in help
const ABOUT_ROOT: &'static str = r##"
Personal multi tool

    A collection of tools made for personal use
"##;
/// longer about section displayed in help, is combined with [the short help](ABOUT_ROOT)
static LONG_ABOUT_ROOT: &'static str = r##"

    libpt is a personal general purpose library, offering this executable, a python module and a
    dynamic library.
"##;

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////
/// ## Main struct for parsing CLI arguments
///
/// This struct describes the complete commandline options/arguments that [pt](crate) can take. It
/// makes use of composition to build a complex system of commands, subcommands, flags and options.
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

    /// choose a subcommand
    #[command(subcommand)]
    pub command: Commands,
}

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////
/// ## defines the top level commands
#[derive(Debug, Clone, Subcommand)]
#[non_exhaustive]
pub enum Commands {
    /// networking commands
    Net {
        /// Networking subcommands
        #[command(subcommand)]
        command: NetCommands,
    },
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// ## defines the networking commands
#[derive(Debug, Clone, Subcommand)]
#[non_exhaustive]
pub enum NetCommands {
    /// monitor your network
    Monitor {
        /// repeat every N seconds, 0 means no repeat
        #[clap(short, long, default_value_t = 0, name = "N")]
        repeat: u64,

        /// At what percentage should the try be considered successful
        #[clap(short, long, default_value_t = 100, value_parser=max100)]
        success_ratio: u8,

        /// extra URLs to check with
        extra_urls: Vec<String>,

        /// Don't check for default URLs
        #[clap(short, long)]
        no_default: bool

    },
    /// discover hosts in your network
    Discover {

    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////
/// custom value parser, only allow 0 to 100
fn max100(s: &str) -> Result<u8, String> {
    number_range(s, 0, 100)
}
