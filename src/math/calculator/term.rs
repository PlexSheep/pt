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
use crate::logger::*;

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
    Operator(Operator),
    /// A concrete value that we can calculate something with. May be a constant, integer, float,
    /// etc.
    Value(super::base::Value),
    /// A variable of some kind that will be present in the result
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
    /// the calculated result, may be of diffrent types, see [`crate::math::calculator::result`].
    pub result: Option<Result<Value>>,
    /////////////////////////////////////
    ///// internal values following /////
    /////////////////////////////////////
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
                result: None,
                operator_stack: Vec::new(),
                output_queue: VecDeque::new()
            }
        )
    }

    /// Prepare the term for the processing.
    pub fn prepare(&mut self) -> Result<()> {
        // TODO: shunting yard

        // Storage for unfinished tokens
        let mut unfinished_chars: Vec<char> = Vec::new();

        for c in self.original.chars() {
            // FIXME: this completely ignores shunting yard, 
            // only being on the lookout for values
            if Self::is_tok(&unfinished_chars) {
                let tok = Self::to_tok(unfinished_chars)?;
                // TODO: handle the token, depending on type, precedence and so on
                self.output_queue.push_front(tok);
                unfinished_chars = Vec::new();
            }
            else {
                unfinished_chars.push(c);
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
    fn to_tok(s: Vec<char>) -> Result<Token> {
        Ok(19.into())
    }

    fn is_tok(s: &Vec<char>) -> bool {
        false
    }
}

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
