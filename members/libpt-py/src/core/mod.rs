use pyo3::prelude::*;

mod printing;

/// implement a python module in Rust
pub fn submodule(py: Python, parent: &PyModule) -> PyResult<()> {
    let module = PyModule::new(py, "core")?;
    printing::submodule(py, module)?;
    parent.add_submodule(module)?;
    Ok(())
}
