//! Python bindings for [`libpt`]

#[cfg(feature = "bintols")]
mod bintols;
#[cfg(feature = "core")]
mod core;
#[cfg(feature = "log")]
mod log;

use pyo3::prelude::*;

/// return the version of libpt
#[pyfunction]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// implement a python module in Rust
#[pymodule]
#[pyo3(name = "libpt")]
fn libpt_py(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(version, m)?)?;
    #[cfg(feature = "core")]
    core::submodule(py, m)?;
    #[cfg(feature = "log")]
    log::submodule(py, m)?;
    #[cfg(feature = "bintols")]
    bintols::submodule(py, m)?;
    Ok(())
}
