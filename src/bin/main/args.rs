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
use clap::{Args, Parser, Subcommand};

use clap_num::number_range;

use clap_verbosity_flag::Verbosity;

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
    /// set a verbosity, multiple allowed (f.e. -vvv)
    #[command(flatten)]
    pub verbose: Verbosity,

    /// choose a subcommand
    ///
    ///
    #[command(subcommand)]
    pub command: Commands,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Args)]
pub struct NetDiscoverArgs {
    #[clap(short)]
    test: bool,
}

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////
/// # Top level commands
#[derive(Debug, Clone, Subcommand)]
#[non_exhaustive]
pub enum Commands {
    /// networking commands
    Net {
        #[command(subcommand)]
        command: NetCommands,
    },
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Subcommand)]
#[non_exhaustive]
pub enum NetCommands {
    /// monitor your network
    Monitor {
        #[clap(short, long)]
        repeat: bool,

        #[clap(short, long, default_value_t = 100, value_parser=max100)]
        percentage_for_success: u8,

        #[arg(default_values_t = ["https://cloudflare.com".to_string()])]
        additional_domains: Vec<String>,

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
