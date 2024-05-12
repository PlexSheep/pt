use pyo3::prelude::*;

use libpt::bintols as origin;

mod split {
    use libpt::bintols::split as origin;
    use pyo3::prelude::*;

    #[pyfunction]
    pub fn split_int(data: u128) -> Vec<u8> {
        origin::unsigned_to_vec(data)
    }

    /// implement a python module in Rust
    pub fn submodule(py: Python, parent: &PyModule) -> PyResult<()> {
        let module = PyModule::new(py, "split")?;

        module.add_function(wrap_pyfunction!(split_int, module)?)?;

        parent.add_submodule(module)?;
        Ok(())
    }
}

mod display {
    use libpt::bintols::display as origin;
    use pyo3::prelude::*;

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
    split::submodule(py, module)?;

    parent.add_submodule(module)?;
    Ok(())
}
