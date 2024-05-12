use std::path::PathBuf;

use pyo3::prelude::*;

use libpt::log as origin;

#[derive(Clone)]
#[pyclass]
pub enum Level {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<Level> for origin::Level {
    fn from(value: Level) -> Self {
        match value {
            Level::Error => origin::Level::ERROR,
            Level::Warn => origin::Level::WARN,
            Level::Info => origin::Level::INFO,
            Level::Debug => origin::Level::DEBUG,
            Level::Trace => origin::Level::TRACE,
        }
    }
}

#[pyclass]
pub struct Logger {
    inner: origin::Logger,
}

impl From<origin::Logger> for Logger {
    fn from(inner: origin::Logger) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Logger {
    #[new]
    pub fn build(
        log_dir: Option<PathBuf>,
        max_level: Option<Level>,
        uptime: Option<bool>,
    ) -> anyhow::Result<Self> {
        // concert our wrapper type
        let max_level = max_level.map(origin::Level::from);
        let mut builder = origin::Logger::builder();
        if log_dir.is_some() {
            builder = builder.log_dir(log_dir.unwrap());
        }
        if max_level.is_some() {
            builder = builder.max_level(max_level.unwrap());
        }
        if uptime.is_some() {
            builder = builder.uptime(uptime.unwrap());
        }
        Ok(builder.build()?.into())
    }

    /// ## logging at [`Level::ERROR`]
    pub fn error(&self, printable: String) {
        self.inner.error(printable)
    }
    /// ## logging at [`Level::WARN`]
    pub fn warn(&self, printable: String) {
        self.inner.warn(printable)
    }
    /// ## logging at [`Level::INFO`]
    pub fn info(&self, printable: String) {
        self.inner.info(printable)
    }
    /// ## logging at [`Level::DEBUG`]
    pub fn debug(&self, printable: String) {
        self.inner.debug(printable)
    }
    /// ## logging at [`Level::StringRACE`]
    pub fn trace(&self, printable: String) {
        self.inner.trace(printable)
    }
}

/// implement a python module in Rust
pub fn submodule(py: Python, parent: &PyModule) -> PyResult<()> {
    let module = PyModule::new(py, "log")?;
    module.add_class::<Logger>()?;
    parent.add_submodule(module)?;
    Ok(())
}
