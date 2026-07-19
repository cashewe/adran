use pyo3::prelude::*;

/// Return a greeting for the given name.
#[pyfunction]
fn greet(name: &str) -> PyResult<String> {
    Ok(format!("Hello, {}!", name))
}

/// Main Python module for `adran`.
#[pymodule]
fn adran(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    Ok(())
}
