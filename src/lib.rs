//! # root module of `libpt`
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
pub mod common;
/// logger used by libpt
pub mod logger;
/// networking tools
pub mod networking;
use crate::logger::Logger;

use pyo3::prelude::*;

//// PUBLIC FUNCTIONS //////////////////////////////////////////////////////////////////////////////
/// ## Check if [`libpt`](crate) has been loaded
///
/// Always returns `true` if you can execute it.
#[pyfunction]
pub fn is_loaded() -> bool {
    true
}

//// PRIVATE FUNCTIONS /////////////////////////////////////////////////////////////////////////////
/// ## Python module: logger
#[pymodule]
fn py_logger(py: Python, m: &PyModule) -> PyResult<()> {
    let logger_module = PyModule::new(py, "logger")?;
    logger_module.add_class::<Logger>()?;

    m.add_submodule(logger_module)?;
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// ## Python module: root
///
/// This function is the entry point of [`PyO3`](pyo3). This is where the main module is built.
#[pymodule]
fn _libpt(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_loaded, m)?)?;

    // load logger module
    py_logger(py, m)?;
    Ok(())
}
