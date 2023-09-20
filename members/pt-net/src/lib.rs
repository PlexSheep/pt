//! # various networking tools
//!
//! The networking module contains various tools related to connections. For example, it contains a
//! tool that has the purpose to check if your connection is consistently available.

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
/// monitor your connection
pub mod monitoring;

//// CONSTANTS /////////////////////////////////////////////////////////////////////////////////////
/// how long to wait for a remove server to respond in ms
pub const DEFAULT_REQUEST_TIMEOUT: u64 = 2000;

//// STATICS ///////////////////////////////////////////////////////////////////////////////////////

//// MACROS ////////////////////////////////////////////////////////////////////////////////////////

//// ENUMS /////////////////////////////////////////////////////////////////////////////////////////

//// STRUCTS ///////////////////////////////////////////////////////////////////////////////////////

//// IMPLEMENTATION ////////////////////////////////////////////////////////////////////////////////

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////