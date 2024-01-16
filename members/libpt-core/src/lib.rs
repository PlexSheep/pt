//! # common functionalities
//!
//! This crate is part of [`pt`](../libpt/index.html), but can also be used as a standalone
//! module.
//!
//! This crate implements core functionality useful for many use cases, such as macros,
//! formatting functions and more.

//// ATTRIBUTES ////////////////////////////////////////////////////////////////////////////////////
// we want docs
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
////////////////////////////////////////////////////////////////////////////////////////////////////
// we want Debug everywhere.
#![warn(missing_debug_implementations)]
////////////////////////////////////////////////////////////////////////////////////////////////////
// enable clippy's extra lints, the pedantic version
#![warn(clippy::pedantic)]

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
/// macros to make things faster in your code
pub mod macros;
/// some general use printing to stdout tools
pub mod printing;

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////