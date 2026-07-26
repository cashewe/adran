pub mod _types;
pub mod md_parser;

use crate::md_parser::MDParser;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn parse_markdown(source: &str) -> PyResult<String> {
    let doc = MDParser::new()
        .parse(source)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    serde_json::to_string(&doc).map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pymodule]
fn adran(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_markdown, m)?)?;
    Ok(())
}