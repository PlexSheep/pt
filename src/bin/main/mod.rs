//! # Main executable of pt
//!
//! This module contains all code specific to the executable version of [`libpt`]: `pt`.

//// ATTRIBUTES ////////////////////////////////////////////////////////////////////////////////////
// we want docs
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
// we want Debug everywhere.
#![warn(missing_debug_implementations)]
// enable clippy's extra lints, the pedantic version
#![warn(clippy::pedantic)]

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
use libpt::networking::monitoring::uptime;
use libpt::logger;

// we want the log macros in any case
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use env_logger;

use clap::Parser;

mod args;
use args::*;

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////

//// MACROS ////////////////////////////////////////////////////////////////////////////////////////

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////
/// ## Main function of the `pt` binary
fn main() {
    #[cfg(debug_assertions)]
    std::env::set_var(logger::LOGGER_ENV_KEY, "trace");

    let cli = Cli::parse();
    // set up our logger to use the given verbosity
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    trace!("started the main function");
    trace!("{:?}", &cli);

    match cli.clone().command {
        Commands::Net { command } => net(&cli, command),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// ## Process `Net` subcommands
fn net(cli: &Cli, command: NetCommands) {
    match command {
        NetCommands::Monitor {
            repeat,
            percentage_for_success,
            additional_domains,
        } => {
            let status: uptime::UptimeStatus = uptime::check_status(
                additional_domains,
                percentage_for_success,
            );
            let _verbose = cli.verbose.log_level().is_some();
            println!("{}", uptime::display_uptime_status(status));

        }
        NetCommands::Discover {} => {}
    }
}
