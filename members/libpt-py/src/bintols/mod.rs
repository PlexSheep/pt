use pyo3::prelude::*;

use libpt::bintols as origin;

mod display {
    use pyo3::prelude::*;

    use libpt::bintols::display as origin;

    #[pyfunction]
    pub fn bytes_to_bin(data: &[u8]) -> String {
        origin::bytes_to_bin(data)
    }

    #[pyfunction]
    pub fn byte_bit_display(data: usize) -> String {
        origin::byte_bit_display(data)
    }

    #[pyfunction]
    pub fn humanbytes(total: u128) -> String {
        origin::humanbytes(total)
    }

    /// implement a python module in Rust
    pub fn submodule(py: Python, parent: &PyModule) -> PyResult<()> {
        let module = PyModule::new(py, "display")?;

        module.add_function(wrap_pyfunction!(bytes_to_bin, module)?)?;
        module.add_function(wrap_pyfunction!(byte_bit_display, module)?)?;
        module.add_function(wrap_pyfunction!(humanbytes, module)?)?;

        parent.add_submodule(module)?;
        Ok(())
    }
}

/// implement a python module in Rust
pub fn submodule(py: Python, parent: &PyModule) -> PyResult<()> {
    let module = PyModule::new(py, "bintols")?;

    // binary constants
    module.add("KIBI", origin::KIBI)?;
    module.add("MEBI", origin::MEBI)?;
    module.add("GIBI", origin::GIBI)?;
    module.add("TEBI", origin::TEBI)?;
    module.add("PEBI", origin::PEBI)?;
    module.add("EXBI", origin::EXBI)?;
    module.add("ZEBI", origin::ZEBI)?;
    module.add("YOBI", origin::YOBI)?;

    display::submodule(py, module)?;

    parent.add_submodule(module)?;
    Ok(())
}
