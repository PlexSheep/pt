//! # common macros for `libpt`
//!
//! This module implements macros for use with `libpt`.

pub use crate::get_stdout_for;

/// ## catches what the expression would write to the `stdout`
///
/// This macro takes an expression, executes it, and catches what it would write to the stdout.
/// The buffer of the stdout will then be returned for further use.
///
/// This is especially useful when testing loggers or other frontend CLI functions.
#[macro_export]
macro_rules! get_stdout_for {
    ($test:expr) => {{
        use gag::BufferRedirect;
        use std::io::Read;

        let mut buf = BufferRedirect::stdout().unwrap();

        $test;

        let mut output = String::new();
        buf.read_to_string(&mut output).unwrap();
        drop(buf);

        output
    }};
}
