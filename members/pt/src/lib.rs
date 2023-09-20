//! # `libpt`
//!
//! [`libpt`](crate) contains my personal code. It is compiled as all of the following:
//!
//! - dynamic library (`cdylib`, `.so` file on Linux)
//! - rust library crate (`rlib`, usable as )
//! - python module (with [`PyO3`](pyo3))
//! - executable (as `pt`)
//!
//! For more info on the linkage types, please refer to the
//! [rust reference](https://doc.rust-lang.org/reference/linkage.html).

//// ATTRIBUTES ////////////////////////////////////////////////////////////////////////////////////
// we want docs
#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
// we want Debug everywhere. This is a library and there will be many bugs.
#![warn(missing_debug_implementations)]
// enable clippy's extra lints, the pedantic version
#![warn(clippy::pedantic)]

//// IMPORTS ///////////////////////////////////////////////////////////////////////////////////////
/// contains useful code, such as macros, for general use
pub use pt_core;
pub use pt_log;
pub use pt_net;
pub use pt_hedu;
