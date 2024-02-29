//! Python bindings for [`libpt`](libpt)
use libpt;

#[cfg(feature = "core")]
mod core;

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
    Ok(())
}
