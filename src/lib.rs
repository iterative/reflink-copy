#[pyo3::pymodule(gil_used = false)]
mod reflink_copy {
    use std::path::PathBuf;

    #[pyo3::pyfunction]
    fn reflink(src: PathBuf, dst: PathBuf) -> std::io::Result<()> {
        reflink_copy::reflink(src, dst)
    }

    #[pyo3::pyfunction]
    fn reflink_or_copy(src: PathBuf, dst: PathBuf) -> std::io::Result<()> {
        reflink_copy::reflink_or_copy(src, dst).map(|_| ())
    }
}
