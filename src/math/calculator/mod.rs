//! # Calculate expressions
//!
//! Calculate Calculations with your Calculator (`ccc`)
//!
//! This modules aim is to take a term of any kind ([String]) and calculate it's value, be it
//! variable based or a concrete numerical value. It implements different operators and
//! (mathematical) functions.

//// ATTRIBUTES ////////////////////////////////////////////////////////////////////////////////////
// we want docs
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
// we want Debug everywhere.
#![warn(missing_debug_implementations)]
// enable clippy's extra lints, the pedantic version
#![warn(clippy::pedantic)]

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
pub mod result;
pub use result::{Error, Result, CalculateResult};

#[allow(unused_imports)]    // we possibly want to use all log levels
use crate::logger::{trace, debug, info, warn, error};

//// TYPES /////////////////////////////////////////////////////////////////////////////////////////

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////

//// MACROS ////////////////////////////////////////////////////////////////////////////////////////

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////
// #[non_exhaustive]
// #[derive(Debug)]
// pub enum Constants {
//     Pi
// }

////////////////////////////////////////////////////////////////////////////////////////////////////
#[non_exhaustive]
#[derive(Debug)]
/// ## Supported Operations
///
/// This `enum` contains all operations supported in this module.
pub enum Operations {
    /// Mathmatical addition
    Addition,
    /// Mathmatical subtraction
    Subtraction,
    /// Mathmatical multiplication
    Multiplication,
    /// Mathmatical division
    Division,
    /// Mathmatical modulo, finite field arithmetic
    Modulo,
    /// Any function, seel [`Function`]
    Function(Function)

}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// ## Supported Functions
///
/// This `enum` contains all functions supported in this module.
///
/// A function has a name followed by braces directly afterwards.
/// A function may have 0 to 31 Arguments.
///
/// Example: `sqrt(19)`, `floor(19.9)`
#[non_exhaustive]
#[derive(Debug)]
pub enum Function {
    /// Draw the mathmatical root, attribute n is the nth root
    Root(u16),
    /// round up
    Floor,
    /// round down
    Ceil,
    /// round to nearest integer
    /// (commercial rounding)
    Round,
}

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////
/// ## A Calculator object
pub struct Calculator;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Term {
    original: String,
    result: Option<CalculateResult>,
    parts: Vec<String>
}

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////
impl Calculator {
    pub fn oneshot(t: String) -> Result<CalculateResult> {
        trace!(orig=t, "parsing original string to Term");
        let mut t = Term::new(t);
        trace!("term has been parsed, starting Calculation");
        debug!("parsed term: {t:#?}");
        Self::calc(t)
    }

    /// ## Calculate a [`Term`]
    ///
    /// This method makes use of the 
    /// [shunting yard algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm) to
    /// Calculate the final value of any term.
    ///
    /// This method only processes a single term at a time, without caching.
    pub fn calc(mut t: Term) -> Result<CalculateResult> {
        trace!("Calculating term {t:?}");
        return Ok(CalculateResult::from(0))
    }
}

impl Term {
    pub fn new(orig: String) -> Self {
        Term {
            original: orig,
            result: None,
            parts: Vec::new()
        }
    }
}

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
