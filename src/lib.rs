#[pyo3::pymodule]
mod reflink_copy {
    use pyo3::pyfunction;
    use std::path::PathBuf;

    #[pyfunction]
    fn reflink(src: PathBuf, dst: PathBuf) -> pyo3::PyResult<()> {
        Ok(reflink_copy::reflink(src, dst)?)
    }

    #[pyfunction]
    fn reflink_or_copy(src: PathBuf, dst: PathBuf) -> pyo3::PyResult<()> {
        Ok(reflink_copy::reflink_or_copy(src, dst).map(|_| ())?)
    }
}
