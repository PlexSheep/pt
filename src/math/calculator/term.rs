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
pub use super::{Error, Result, CalculateResult};

//// TYPES /////////////////////////////////////////////////////////////////////////////////////////

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////

//// MACROS ////////////////////////////////////////////////////////////////////////////////////////

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////
/// ## Supported Operations
///
/// This `enum` contains all operations supported in this module.
#[non_exhaustive]
#[derive(Debug)]
pub enum Operator {
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
    Value(),
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
    pub result: Option<CalculateResult>,
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
    pub fn prepare(&mut self) {
        // TODO: shunting yard
    }

    pub fn process(&mut self) {
        // TODO: process RPN and set result
        self.result = Some(CalculateResult::Numerical(19.into()))
    }
}


//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
