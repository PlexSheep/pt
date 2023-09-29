//! # A term that can be the input for calculation
//!
//! Short description
//!
//! Details
//!
//! ## Section 1
//!
//! ## Section 2

//// ATTRIBUTES ////////////////////////////////////////////////////////////////////////////////////
// we want docs
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
// we want Debug everywhere.
#![warn(missing_debug_implementations)]
// enable clippy's extra lints, the pedantic version
#![warn(clippy::pedantic)]

use std::collections::VecDeque;

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
pub use super::{Error, Result, Value, base::{self, *}};
#[allow(unused_imports)]    // we possibly want to use all log levels
use libpt_log::*;
#[allow(unused_imports)]    // import more complex math stuff from there
use libpt_math;

//// TYPES /////////////////////////////////////////////////////////////////////////////////////////

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////

//// MACROS ////////////////////////////////////////////////////////////////////////////////////////

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
/// ## Parsed value to be calculated
///
/// This enum represents anything that goes to the output queue of [`Term::prepare()`] and will
/// then be used to actually calculate something in [`Term::process()`].
#[derive(Debug)]
enum Token {
    /// Some kind of operator
    #[allow(unused)] // tmp
    Operator(Operator),
    /// A concrete value that we can calculate something with. May be a constant, integer, float,
    /// etc.
    /// The Token has a value that can be used in calculation
    Value(super::base::Value),
    /// A variable of some kind that will be present in the result
    #[allow(unused)] // tmp
    Variable(char),
}
//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////
/// ## Term that can be calculated
///
/// Represents a signular term, that can be calculated. Terms will be evaluated by the [`Term::prepare`]
/// function, afterwards calculated (as much as possible) in the [`Term::process`] function.
///
#[derive(Debug)]
pub struct Term {
    /// the original expression to calculate
    pub original: String,
    /// the filtered text of the expression, only with relevant information
    pub text: String,
    /// the calculated result, may be of diffrent types, see [`crate::math::calculator::result`].
    pub result: Option<Result<Value>>,
    /////////////////////////////////////
    ///// internal values following /////
    /////////////////////////////////////
    #[allow(unused)] // tmp
    operator_stack: Vec<Operator>,
    output_queue: VecDeque<Token>
}

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////
impl Term {
    /// Build a new term from an expression
    ///
    /// Invalid terms will result in an [`Err`].
    pub fn new(orig: String) -> Result<Term> {
        return Ok(
            Term {
                original: orig,
                text: String::new(), // will be initialized in `prepare()`
                result: None,
                operator_stack: Vec::new(),
                output_queue: VecDeque::new()
            }
        )
    }

    /// Prepare the term for the processing.
    pub fn prepare(&mut self) -> Result<()> {
        trace!("preparing term: {:#?}", self);
        self.text = Self::filter(&self.original)?;

        // Storage for unfinished tokens
        let _unfinished_chars: Vec<char> = Vec::new();

        for (_index, c) in self.original.chars().enumerate() {
            // this will be a mess, but it has to be before i can sort the mess.
            match c {
                // TODO: make function to check if character is an operator, use it
                _ => {
                    let reason = format!("The meaning of '{c}' could not be identified.");
                    warn!(reason);
                    return Err(Error::SyntaxError(reason));
                }
            }
        }
        Ok(())
    }

    /// Process a prepared term, calculating it's result
    pub fn process(&mut self) -> Result<()> {
        debug!("processing term: {:#?}", self);
        debug!("queue: {:#?}", self.output_queue);
        // TODO: process RPN and set result
        self.result = Some(Ok(19.into()));
        Ok(())
    }

    /// Convert a character into a token
    ///
    /// Returns: A tuple with a [`Token`] and a [`bool`]. If the bool is false, the [`Token`] is
    /// marked as "incomplete", meaning that the character cannot be used yet.
    #[allow(unused)] // tmp
    fn to_tok(_s: Vec<char>) -> Result<Token> {
        Ok(19.into())
    }

    /// only leave relevant chars for calculation
    // TODO: make function to check if character is an operator, use it
    fn filter(s: &str) -> Result<String> {
        // pre checks
        // NOTE: Apperently, "alphanumeric" in Rust is a pretty broad term.
        // Even CJK characters or Japanese Kana are allowed:
        // - 'さ' alphanumeric
        // - '数' alphanumeric
        // - '学' alphanumeric
        // - '+'  not alphanumeric
        for c in s.chars() {
            #[cfg(debug_assertions)] {
                debug!("filter checks for '{c}':
                alphanumeric:       {}
                allowed special:    {}
                EXCEPT IF
                ascii control:      {}
                ",
                !c.is_alphanumeric(),
                !Self::is_allowed_special_c(&c),
                c.is_ascii_control(),
            )
            }
            if
                (
                    !c.is_alphanumeric()                    ||
                    !Self::is_allowed_special_c(&c)
                )
                &&
                (
                    c.is_ascii_control()
                )
            {
                // TODO: allow any unicode char to be a variable
                let reason = format!("'{c}' is not a valid character, only alphanumeric, punctuation, operators are allowed.");
                warn!(reason);
                return Err(Error::SyntaxError(reason));
            }
        }

        // filter out single chars
        let mut filtered = String::new();
        for c in s.chars() {
            if !Self::is_ignore(&c) {
                filtered.push(c);
            }
        }

        return Ok(filtered)
    }

    /// check if we should ignore this character
    fn is_ignore(c: &char) -> bool {
        match *c {
            ' ' => true,
            _ => false
        }
    }

    /// allowed special chars
    fn is_allowed_special_c(c: &char) -> bool {
        match *c {
            '+' | '-' | '*' | '/' | '%' => true,
            _ => false
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Helper methods for Tokens
impl Token { }

////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T> From<T> for Token where
    T: Into<Value>,
    T: PrimInt,
    u128: TryFrom<T>
    {
    fn from(value: T) -> Self {
        Token::Value(base::Value::from(value))
    }
}

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
