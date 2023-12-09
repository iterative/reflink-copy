use pyo3::prelude::*;
use std::path::PathBuf;

#[pyfunction]
fn reflink(src: PathBuf, dst: PathBuf) -> PyResult<()> {
    Ok(reflink_copy::reflink(src, dst)?)
}

#[pyfunction]
fn reflink_or_copy(src: PathBuf, dst: PathBuf) -> PyResult<()> {
    Ok(reflink_copy::reflink_or_copy(src, dst).map(|_| ())?)
}

#[pymodule]
fn glitters(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reflink, m)?)?;
    m.add_function(wrap_pyfunction!(reflink_or_copy, m)?)?;
    Ok(())
}
