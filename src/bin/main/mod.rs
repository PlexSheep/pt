//! # Main executable of pt
//!
//! This module contains all code specific to the executable version of [`pt`]: [`pt`](crate).
//!
//!

//// ATTRIBUTES ////////////////////////////////////////////////////////////////////////////////////
// we want docs
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
// we want Debug everywhere.
#![warn(missing_debug_implementations)]
// enable clippy's extra lints, the pedantic version
#![warn(clippy::pedantic)]

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
use pt::networking::monitoring::uptime;

// we want the log macros in any case
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use env_logger;

use clap::Parser;

pub mod args;
use args::*;

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////

//// MACROS ////////////////////////////////////////////////////////////////////////////////////////

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////
/// ## Main function of the [`pt`](crate) binary
pub fn main() {
    let cli = Cli::parse();
    if cli.log_meta {
        // set up our logger to use the given verbosity
        env_logger::Builder::new()
            .filter_module("pt", cli.verbose.log_level_filter())
            .init();
    }
    else {
        // set up our logger to use the given verbosity
        env_logger::Builder::new()
            .filter_module("pt", cli.verbose.log_level_filter())
            .format_level(false)
            .format_target(false)
            .format_timestamp(None)
            .init();
    }

    trace!("started the main function");
    trace!("{:?}", &cli);

    match cli.clone().command {
        Commands::Net { command } => net(&cli, command),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// ## Process [`Net`](args::NetCommands) subcommands
pub fn net(cli: &Cli, command: NetCommands) {
    match command {
        NetCommands::Monitor {
            repeat,
            success_ratio,
            extra_urls,
            no_default,
            timeout
        } => {
            let urls: Vec<String>;
            if no_default {
                urls = extra_urls;
            }
            else {
                let mut combined: Vec<String> = Vec::new();
                for i in uptime::DEFAULT_CHECK_URLS {
                    combined.push(i.to_string());
                }
                combined.extend(extra_urls);
                urls = combined;
            }
            let _verbose = cli.verbose.log_level().is_some();
            if repeat > 0 {
                uptime::continuous_uptime_monitor(success_ratio, urls, repeat * 1000, timeout);
            } else {
                    let status = uptime::UptimeStatus::new(success_ratio, urls, timeout);
                    println!("{}", status);
            }
        }
        NetCommands::Discover {} => {todo!()}
    }
}

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////
