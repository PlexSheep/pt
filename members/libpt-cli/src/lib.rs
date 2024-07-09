//! This module bundles a lot of good CLI tools, and adds some of it's own, to make development of
//! CLI apps easier and more ergonomic.
#![warn(clippy::pedantic, clippy::style, clippy::nursery)]
pub mod args;
pub mod printing;
pub mod repl;

pub use clap;
pub use comfy_table;
pub use console;
pub use dialoguer;
pub use exitcode;
pub use human_panic;
pub use indicatif;
pub use shlex;
pub use strum;
