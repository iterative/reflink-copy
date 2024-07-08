#[pyo3::pymodule]
mod reflink_copy {
    use std::path::PathBuf;

    #[pyo3::pyfunction]
    fn reflink(src: PathBuf, dst: PathBuf) -> pyo3::PyResult<()> {
        Ok(reflink_copy::reflink(src, dst)?)
    }

    #[pyo3::pyfunction]
    fn reflink_or_copy(src: PathBuf, dst: PathBuf) -> pyo3::PyResult<()> {
        Ok(reflink_copy::reflink_or_copy(src, dst).map(|_| ())?)
    }
}
