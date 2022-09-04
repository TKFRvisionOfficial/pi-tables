use std::io::Cursor;
use pyo3::exceptions::PyValueError;
use pyo3::impl_::pyfunction::wrap_pyfunction;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyfunction]
fn conjure_a_table(input_image: &PyBytes) -> PyResult<Vec<Vec<String>>> {
    let buf = input_image.as_bytes();
    let cursor = Cursor::new(buf);
    match itables::extract_table_png_buf(cursor) {
        Ok(table) => Ok(table),
        Err(err) => Err(PyValueError::new_err(err.to_string()))
    }
}

#[pymodule]
fn pitables(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(conjure_a_table, m)?)?;
    Ok(())
}
