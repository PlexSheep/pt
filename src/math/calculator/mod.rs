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
pub mod base;
pub use base::{Error, Result, Value};
pub mod term;
pub use term::*;

#[allow(unused_imports)]    // we possibly want to use all log levels
use crate::logger::{trace, debug, info, warn, error};

//// TYPES /////////////////////////////////////////////////////////////////////////////////////////

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////

//// MACROS ////////////////////////////////////////////////////////////////////////////////////////

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////
/// ## A Calculator struct
///
/// This struct does not do anything at the moment, but aims to be the target for high level
/// control. Instead of using the [`Calculator`], you could just use the [`Term`] struct for
/// lower level control.
pub struct Calculator;

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////
impl Calculator {
    /// Do a single calculation without doing anything else
    pub fn oneshot(t: String) -> Result<Value> {
        trace!(orig=t, "parsing original string to Term");
        let t = Term::new(t)?;
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
    pub fn calc(mut t: Term) -> Result<Value> {
        trace!("Calculating term {t:?}");
        t.prepare()?;
        t.process()?;
        if t.result.is_none() {
            error!("Term was processed but no result was assigned.");
            return Err(Error::SyntaxError)
        }
        return t.result.unwrap()
    }
}

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////////////////////////
