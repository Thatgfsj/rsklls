//! PyO3 bindings for rsklls

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// Initialize PyO3 module
#[cfg(feature = "pyo3")]
#[pymodule]
pub fn rsklls_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

/// PyO3 skill wrapper
#[cfg(feature = "pyo3")]
#[pyclass]
pub struct PySkill {
    name: String,
    description: String,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl PySkill {
    #[new]
    fn new(name: String, description: String) -> Self {
        Self { name, description }
    }
    
    fn __repr__(&self) -> String {
        format!("PySkill(name='{}', description='{}')", self.name, self.description)
    }
}
