//! This module implements a default repl that fullfills the [Repl] trait
//!
//! You can implement your own [Repl] if you want.

use std::fmt::Debug;

use super::Repl;

use embed_doc_image::embed_doc_image;

/// [clap] help template with only usage and commands/options
pub const REPL_HELP_TEMPLATE: &str = r"{usage-heading} {usage}

{all-args}{tab}
";

use clap::{Parser, Subcommand};
use dialoguer::{BasicHistory, Completion};
use libpt_log::trace;

#[allow(clippy::needless_doctest_main)] // It makes the example look better
/// Default implementation for a REPL
///
/// Note that you need to define the commands by yourself with a Subcommands enum.
///
/// # Example
///
/// ```no_run
/// use libpt_cli::repl::{DefaultRepl, Repl};
/// use clap::Subcommand;
/// use strum::EnumIter;
///
/// #[derive(Subcommand, Debug, EnumIter, Clone)]
/// enum ReplCommand {
///     /// hello world
///     Hello,
///     /// leave the repl
///     Exit,
/// }
///
/// fn main() {
///     let mut repl = DefaultRepl::<ReplCommand>::default();
///     loop {
///         repl.step().unwrap();
///         match repl.command().to_owned().unwrap() {
///             ReplCommand::Hello => println!("Hello"),
///             ReplCommand::Exit => break,
///             _ => (),
///         }
///     }
/// }
/// ```
/// **Screenshot**
///
/// ![Screenshot of an example program with a REPL][repl_screenshot]
#[embed_doc_image("repl_screenshot", "data/media/repl.png")]
#[derive(Parser)]
#[command(multicall = true, help_template = REPL_HELP_TEMPLATE)]
#[allow(clippy::module_name_repetitions)] // we can't just name it `Default`, that's part of std
pub struct DefaultRepl<C>
where
    C: Debug + Subcommand + strum::IntoEnumIterator,
{
    /// the command you want to execute, along with its arguments
    #[command(subcommand)]
    command: Option<C>,

    // the following fields are not to be parsed from a command, but used for the internal workings
    // of the repl
    #[clap(skip)]
    buf: String,
    #[clap(skip)]
    buf_preparsed: Vec<String>,
    #[clap(skip)]
    completion: DefaultReplCompletion<C>,
    #[clap(skip)]
    history: BasicHistory,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct DefaultReplCompletion<C>
where
    C: Debug + Subcommand + strum::IntoEnumIterator,
{
    commands: std::marker::PhantomData<C>,
}

impl<C> Repl<C> for DefaultRepl<C>
where
    C: Debug + Subcommand + strum::IntoEnumIterator,
{
    fn new() -> Self {
        Self {
            command: None,
            buf_preparsed: Vec::new(),
            buf: String::new(),
            history: BasicHistory::new(),
            completion: DefaultReplCompletion::new(),
        }
    }
    fn command(&self) -> &Option<C> {
        &self.command
    }
    fn step(&mut self) -> Result<(), super::error::Error> {
        self.buf.clear();

        // NOTE: display::Input requires some kind of lifetime that would be a bother to store in
        // our struct. It's documentation also uses it in place, so it should be fine to do it like
        // this.
        //
        // NOTE: It would be nice if we could use the Validator mechanism of dialoguer, but
        // unfortunately we can only process our input after we've preparsed it and we need an
        // actual output. If we could set a status after the Input is over that would be amazing,
        // but that is currently not supported by dialoguer.
        // Therefore, every prompt will show as success regardless.
        self.buf = dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
            .completion_with(&self.completion)
            .history_with(&mut self.history)
            .interact_text()?;

        self.buf_preparsed = Vec::new();
        self.buf_preparsed
            .extend(shlex::split(&self.buf).unwrap_or_default());

        trace!("read input: {:?}", self.buf_preparsed);
        trace!("repl after step: {:#?}", self);

        // HACK: find a way to not allocate a new struct for this
        let cmds = Self::try_parse_from(&self.buf_preparsed)?;
        self.command = cmds.command;
        Ok(())
    }
}

impl<C> Default for DefaultRepl<C>
where
    C: Debug + Subcommand + strum::IntoEnumIterator,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C> Debug for DefaultRepl<C>
where
    C: Debug + Subcommand + strum::IntoEnumIterator,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefaultRepl")
            .field("command", &self.command)
            .field("buf", &self.buf)
            .field("buf_preparsed", &self.buf_preparsed)
            .field("completion", &self.completion)
            .field("history", &"(no debug)")
            .finish()
    }
}

impl<C> DefaultReplCompletion<C>
where
    C: Debug + Subcommand + strum::IntoEnumIterator,
{
    /// Make a new [`DefaultReplCompletion`] for the type `C`
    pub const fn new() -> Self {
        Self {
            commands: std::marker::PhantomData::<C>,
        }
    }
    fn commands() -> Vec<String> {
        let mut buf = Vec::new();
        // every crate has the help command, but it is not part of the enum
        buf.push("help".to_string());
        for c in C::iter() {
            // HACK: this is a horrible way to do this
            // I just need the names of the commands
            buf.push(
                format!("{c:?}")
                    .split_whitespace()
                    .map(str::to_lowercase)
                    .next()
                    .unwrap()
                    .to_string(),
            );
        }
        trace!("commands: {buf:?}");
        buf
    }
}

impl<C> Default for DefaultReplCompletion<C>
where
    C: Debug + Subcommand + strum::IntoEnumIterator,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<C> Completion for DefaultReplCompletion<C>
where
    C: Debug + Subcommand + strum::IntoEnumIterator,
{
    /// Simple completion implementation based on substring
    fn get(&self, input: &str) -> Option<String> {
        let matches = Self::commands()
            .into_iter()
            .filter(|option| option.starts_with(input))
            .collect::<Vec<_>>();

        trace!("\nmatches: {matches:#?}");
        if matches.len() == 1 {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}
