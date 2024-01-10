//! # Main executable of libpt
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
use libpt::{log::*, net::monitoring::uptime};

use clap::Parser;

pub mod args;
use args::*;

use std::path::PathBuf;

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////
#[allow(dead_code)]
//// STATICS ///////////////////////////////////////////////////////////////////////////////////////

//// MACROS ////////////////////////////////////////////////////////////////////////////////////////

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////
/// ## Main function of the [`pt`](crate) binary
pub fn main() {
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
            timeout,
        } => {
            let urls: Vec<String>;
            if no_default {
                urls = extra_urls;
            } else {
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
                info!("status:\n{}", status);
            }
        }
        NetCommands::Discover {} => {
            todo!()
        }
    }
}

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////
