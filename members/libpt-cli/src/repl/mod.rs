use std::fmt::Debug;

pub mod error;
use error::ReplError;
mod default;
pub use default::*;

use clap::{Parser, Subcommand};
use dialoguer::Completion;

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
    /// Will only be [None] if the repl has not had [step] executed yet.
    fn command(&self) -> &Option<C>;
    /// return all possible commands in this repl
    fn completion() -> impl Completion;
    /// advance the repl to the next iteration of the main loop
    ///
    /// This should be used at the start of your loop
    fn step(&mut self) -> Result<(), ReplError>;
}
