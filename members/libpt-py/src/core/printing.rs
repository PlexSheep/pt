use pyo3::prelude::*;

use libpt::core::printing as origin;

/// Quickly get a one line visual divider
#[pyfunction]
pub fn divider() -> String {
    origin::divider()
}

/// Quickly print a one line visual divider
#[pyfunction]
pub fn print_divider() {
    origin::print_divider()
}

/// implement a python module in Rust
pub fn submodule(py: Python, parent: &PyModule) -> PyResult<()> {
    let module = PyModule::new(py, "printing")?;
    module.add_function(wrap_pyfunction!(divider, module)?)?;
    module.add_function(wrap_pyfunction!(print_divider, module)?)?;
    parent.add_submodule(module)?;
    Ok(())
}
