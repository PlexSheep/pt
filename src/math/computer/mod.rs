//! # Compute expressions
//!
//! Compute computations with your computer (`ccc`)
//!
//! This modules aim is to take a term of any kind ([String]) and compute it's value, be it
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
pub use result::{Error, Result, ComputeResult};

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
    /// Draw the mathmatical root, attribute n is the nth root
    Root(u16),
    /// round up
    Floor,
    /// round down
    Ceil,
    /// round to nearest integer
    /// (commercial rounding)
    Round
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[non_exhaustive]
pub enum Functions {
    Root
}

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////
pub struct Computer;

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct Term {
    original: String,
    result: Option<ComputeResult>,
    parts: Vec<String>
}

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////
impl Computer {
    pub fn oneshot(t: String) -> Result<ComputeResult> {
        trace!(orig=t, "parsing original string to Term");
        let mut t = Term::new(t);
        trace!("term has been parsed, starting computation");
        debug!("parsed term: {t:#?}");
        Self::compute(t)
    }

    /// ## compute a [`Term`]
    ///
    /// This method makes use of the 
    /// [shunting yard algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm) to
    /// compute the final value of any term.
    ///
    /// This method only processes a single term at a time, without caching.
    pub fn compute(mut t: Term) -> Result<ComputeResult> {
        trace!("computing term {t:?}");
        return Ok(ComputeResult::from(0))
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
