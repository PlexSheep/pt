//! # Results and Errors for the calculate module
//!
//! This module defines the errors and results that can be processed from any given term.

//// ATTRIBUTES ////////////////////////////////////////////////////////////////////////////////////
// we want docs
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
// we want Debug everywhere.
#![warn(missing_debug_implementations)]
// enable clippy's extra lints, the pedantic version
#![warn(clippy::pedantic)]

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
pub use num_traits::PrimInt;
use std::fmt::Display;

#[allow(unused_imports)] // we possibly want to use all log levels
use libpt_log::*;
#[allow(unused_imports)] // import more complex math stuff from there
use libpt_math;

//// TYPES /////////////////////////////////////////////////////////////////////////////////////////
/// Quick Result with a ccc error
pub type Result<T> = std::result::Result<T, Error>;

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
    Function(Function),
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
/// Top Level Error Type
///
/// Contains many variants of other errors, that can occur when using the crate.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// The term has bad syntax
    SyntaxError(String),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Represents some kind of computed value
#[derive(Debug)]
pub enum Value {
    /// Variable value
    Variable(VarVal),
    /// Numerical value
    Numerical(NumVal),
    /// Complex number value
    Complex(ComplVal),
}

/// Represents some kind of numeric value
#[non_exhaustive]
#[derive(Debug)]
pub enum NumVal {
    /// Value > 0
    Signed(i128),
    /// Value can be negative
    Unsigned(u128),
    /// Value is not an integer
    Float(f64),
}

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////
/// Represents a Value with at least one variable,
///
/// currently not implemented
#[derive(Debug)]
pub struct VarVal {}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// Represents a Value with a complex number,
///
/// currently not implemented
#[derive(Debug)]
pub struct ComplVal {}

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////
impl<T: num_traits::PrimInt> From<T> for NumVal
where
    u128: TryFrom<T>,
    u128: TryFrom<T>,
{
    fn from(value: T) -> Self {
        if T::min_value().is_zero() {
            // unsigned data types
            // `u128` is the largest unsigned datatype, any other type will fit.
            NumVal::Unsigned(value.to_u128().unwrap())
        } else {
            // signed data types
            // `i128` is the largest unsigned datatype, any other type will fit.
            NumVal::Signed(value.to_i128().unwrap())
        }
    }
}
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Display Errors with a nice little reason
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SyntaxError(reason) => {
                write!(f, "Syntax Error: {}", reason)
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: PrimInt> From<T> for Value
where
    u128: TryFrom<T>,
    u128: TryFrom<T>,
{
    fn from(value: T) -> Self {
        NumVal::from(value).into()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<NumVal> for Value {
    fn from(value: NumVal) -> Self {
        Value::Numerical(value)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Numerical(val) => {
                write!(f, "{}", val)
            }
            Value::Complex(val) => {
                write!(f, "{}", val)
            }
            Value::Variable(val) => {
                write!(f, "{}", val)
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Display for NumVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumVal::Float(val) => {
                write!(f, "{val}")
            }
            NumVal::Signed(val) => {
                write!(f, "{val}")
            }
            NumVal::Unsigned(val) => {
                write!(f, "{val}")
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Display for ComplVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Display for VarVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
