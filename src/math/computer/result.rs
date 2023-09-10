//! # Results and Errors for the compute module
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
use std::fmt::Display;
use num_traits;

//// TYPES /////////////////////////////////////////////////////////////////////////////////////////
pub type Result<T> = std::result::Result<T, Error>;

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////

//// MACROS ////////////////////////////////////////////////////////////////////////////////////////

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    SyntaxError
}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub enum ComputeResult {
    Variable(VarResult),
    Numerical(NumericResult),
    Complex(ComplexResult),
}

#[non_exhaustive]
#[derive(Debug)]
pub enum NumericResult {
    Signed(i128),
    Unsigned(u128),
    Float(f64)
}

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct VarResult {

}

////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
pub struct ComplexResult {

}

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////
impl<T: num_traits::PrimInt> From<T> for NumericResult where
    u128: TryFrom<T>,
    u128: TryFrom<T> {
    fn from(value: T) -> Self {
        if T::min_value().is_zero() {
            // unsigned data types
            // `u128` is the largest unsigned datatype, any other type will fit.
            NumericResult::Unsigned(value.to_u128().unwrap())
        }
        else {
            // signed data types
            // `i128` is the largest unsigned datatype, any other type will fit.
            NumericResult::Signed(value.to_i128().unwrap())
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl<T: num_traits::PrimInt> From<T> for ComputeResult where
    u128: TryFrom<T>,
    u128: TryFrom<T> {
    fn from(value: T) -> Self {
        NumericResult::from(value).into()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl From<NumericResult> for ComputeResult {
    fn from(value: NumericResult) -> Self {
        ComputeResult::Numerical(value)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Display for ComputeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComputeResult::Numerical(val) => {
                write!(f, "{}", val)
            }
            ComputeResult::Complex(val) => {
                write!(f, "{}", val)
            }
            ComputeResult::Variable(val) => {
                write!(f, "{}", val)
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Display for NumericResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericResult::Float(val) => {
                write!(f, "{val}")
            }
            NumericResult::Signed(val) => {
                write!(f, "{val}")
            }
            NumericResult::Unsigned(val) => {
                write!(f, "{val}")
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Display for ComplexResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
impl Display for VarResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
