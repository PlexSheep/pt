use libpt::{
    log::*,
};

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
fn py_logger(py: Python, parent: &PyModule) -> PyResult<()> {
    let module = PyModule::new(py, "logger")?;
    module.add_class::<Logger>()?;

    parent.add_submodule(module)?;
    Ok(())
}
//
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// /// ## Python module: common
// #[pymodule]
// fn py_common(py: Python, parent: &PyModule) -> PyResult<()> {
//     let module = PyModule::new(py, "common")?;
//     py_common_printing(py, module)?;
//
//     parent.add_submodule(module)?;
//     Ok(())
// }
//
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// /// ## Python module: common.printing
// #[pymodule]
// fn py_common_printing(py: Python, parent: &PyModule) -> PyResult<()> {
//     let module = PyModule::new(py, "printing")?;
//     module.add_function(wrap_pyfunction!(common::printing::divider, module)?)?;
//     module.add_function(wrap_pyfunction!(common::printing::print_divider, module)?)?;
//
//     parent.add_submodule(module)?;
//     Ok(())
// }
//
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// /// ## Python module: networking
// #[pymodule]
// fn py_networking(py: Python, parent: &PyModule) -> PyResult<()> {
//     let module = PyModule::new(py, "networking")?;
//     py_networking_monitoring(py, module)?;
//
//     parent.add_submodule(module)?;
//     Ok(())
// }
//
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// /// ## Python module: networking.monitoring
// #[pymodule]
// fn py_networking_monitoring(py: Python, parent: &PyModule) -> PyResult<()> {
//     let module = PyModule::new(py, "monitoring")?;
//     py_networking_monitoring_uptime(py, module)?;
//
//     parent.add_submodule(module)?;
//     Ok(())
// }
//
// ////////////////////////////////////////////////////////////////////////////////////////////////////
// /// ## Python module: networking.monitoring.uptime
// #[pymodule]
// fn py_networking_monitoring_uptime(py: Python, parent: &PyModule) -> PyResult<()> {
//     let module = PyModule::new(py, "uptime")?;
//     module.add_class::<networking::monitoring::uptime::UptimeStatus>()?;
//     module.add_function(wrap_pyfunction!(
//         networking::monitoring::uptime::py_continuous_uptime_monitor,
//         module
//     )?)?;
//
//     parent.add_submodule(module)?;
//     Ok(())
// }

////////////////////////////////////////////////////////////////////////////////////////////////////
/// ## Python module: root
///
/// This function is the entry point of [`PyO3`](pyo3). This is where the main module is built.
#[pymodule]
fn _libpt(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_loaded, m)?)?;

    // load sub modules
    // py_common(py, m)?;
    py_logger(py, m)?;
    // py_networking(py, m)?;
    Ok(())
}
