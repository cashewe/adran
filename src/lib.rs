pub mod _types;
pub mod md_parser;
pub mod text_recall;

use crate::md_parser::{MDParser, ParsedDocument};
use crate::text_recall::{recall_text_indices, RecallQuery};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyfunction]
fn parse_markdown(source: &str) -> PyResult<String> {
    let doc = MDParser::new()
        .parse(source)
        .map_err(|e| PyValueError::new_err(e.to_string()))?;

    serde_json::to_string(&doc).map_err(|e| PyValueError::new_err(e.to_string()))
}

#[pyfunction]
#[pyo3(signature = (document_json, start, end, text_depth=None, heading_depth=None, text_siblings=false, heading_siblings=false))]
fn run_recall_text_indices(
    document_json: &str,
    start: usize,
    end: usize,
    text_depth: Option<usize>,
    heading_depth: Option<usize>,
    text_siblings: bool,
    heading_siblings: bool,
) -> PyResult<String> {
    let document: ParsedDocument = serde_json::from_str(document_json)
        .map_err(|e| PyValueError::new_err(format!("invalid document JSON: {e}")))?;

    let query = RecallQuery::new(start, end, text_depth, heading_depth, text_siblings, heading_siblings);
    let result = recall_text_indices(&document, &query);

    serde_json::to_string(&result).map_err(|e| PyValueError::new_err(e.to_string()))
}


#[pymodule]
fn adran(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_markdown, m)?)?;
    m.add_function(wrap_pyfunction!(run_recall_text_indices, m)?)?;
    Ok(())
}