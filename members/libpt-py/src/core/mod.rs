use pyo3::prelude::*;

/// implement a python module in Rust
#[pymodule]
#[pyo3(name = "core")]
pub fn submodule(py: Python, m: &PyModule) -> PyResult<()> {
    let submodule = PyModule::new(py, "submodule")?;
    submodule.add("super_useful_constant", "important")?;
    m.add_submodule(m)?;
    Ok(())
}
