//! # Tests for libpt::logger::Logger
//!
//! Note: the module uses a global variable to store if the thread has 
//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
/// ## Tests for basic logging functionality
use libpt::logger::*;
use libpt::common::macros::get_stdout_for;

use regex::Regex;

//// HELPERS ///////////////////////////////////////////////////////////////////////////////////////
// only initialize once
/// ## setup that's needed before testing the logger struct
fn setup() {
    // we don't want to log messages during our tests!
    std::env::set_var(LOGGER_ENV_KEY, "Trace");
    Logger::init_specialized(false, false, env_logger::Target::Stdout);
    println!()
}

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////

/// ## Tests for basic logging
///
/// This test tests if the loggers basic logging functionality works, that is it's methods:
///
/// - [`Logger::trace`]
/// - [`Logger::debug`]
/// - [`Logger::info`]
/// - [`Logger::warn`]
/// - [`Logger::error`]
///
/// After those methods have Successfully been executed, their outputs gets stored in a single
/// [`String`] and a [`Regex`] checks if we have five correctly formatted messages.
#[test]
fn test_log_basic() {
    setup();
    let l = Logger::new();
    let trace_out = get_stdout_for!(l.trace("MSG"));
    let debug_out = get_stdout_for!(l.debug("MSG"));
    let info_out = get_stdout_for!(l.info("MSG"));
    let warn_out = get_stdout_for!(l.warn("MSG"));
    let error_out = get_stdout_for!(l.error("MSG"));
    let combined = format!(
        "{}{}{}{}{}",
        trace_out, debug_out, info_out, warn_out, error_out
    );
    print!("{}", combined);

    // too long, so i split into two lines.
    // this matches the format of the env_logger perfectly, but make sure that color is off,
    // else the ANSI escape sequences break this test
    let regex = Regex::new(concat!(
        r"(?m)\[\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z ",
        r"(TRACE|DEBUG|INFO|WARN|ERROR) +libpt::logger\] MSG"
    ))
    .unwrap();

    // we have 5 log levels, therefore we should have 5 captures
    assert_eq!(regex.captures_iter(&combined).count(), 5);
}

#[test]
fn test_multi_initialize() {
    setup();
    let l = Logger::new();
    // these should be ignored due to the global flag
    Logger::init();
    Logger::init();
    Logger::init();
    Logger::init();
    l.info("Successfully ignored extra init");
}
