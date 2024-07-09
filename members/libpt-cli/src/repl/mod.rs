//! Create easy and well defined REPLs
//!
//! A REPL is a [Read-Eval-Print-Loop](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop).
//! Well known examples for REPLs are shells (like bash).
//!
//! This module offers a convenient way to create a well-defined REPL without a lot of complicated
//! code and with a visually pleasing aesthetic. An example REPL implementation can be found in the
//! examples.
//!
//! The basic idea is that the user defines the commands with an enum and uses [claps](clap)
//! `#[derive(Subcommand)]`. A loop is then used to read from the stdin into a buffer, that buffer
//! is put to [clap] for parsing, similar to how [clap] would parse commandline arguments.

use std::fmt::Debug;

pub mod error;
use error::ReplError;
mod default;
pub use default::*;

use clap::{Parser, Subcommand};

/// Common Trait for repl objects
///
/// Unless you want to implement custom features (not just commands), just use [`DefaultRepl`].
pub trait Repl<C>: Parser + Debug
where
    C: Debug,
    C: Subcommand,
    C: strum::IntoEnumIterator,
{
    /// create a new repl
    fn new() -> Self;
    /// get the command that was parsed from user input
    ///
    /// Will only be [None] if the repl has not had [step](Repl::step) executed yet.
    fn command(&self) -> &Option<C>;
    /// advance the repl to the next iteration of the main loop
    ///
    /// This should be used at the start of your loop.
    ///
    /// Note that the help menu is an Error: [`clap::error::ErrorKind::DisplayHelp`]
    fn step(&mut self) -> Result<(), ReplError>;
}
