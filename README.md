### `reflink-copy`

Python wrapper for [`reflink-copy`](https://github.com/cargo-bins/reflink-copy).

### Installation

You can install the library via `pip` from [PyPI](https://pypi.org/project/reflink-copy):

```console
pip install reflink-copy
```

### Usage

```python
from reflink_copy import reflink, reflink_or_copy

reflink("file1", "file2")
reflink_or_copy("file1", "file2")
```

### Reference

```python
def reflink(src: str | os.PathLike[str], dst: str | os.PathLike[str]) -> None: ...
def reflink_or_copy(src: str | os.PathLike[str], dst: str | os.PathLike[str]) -> None: ...
```

### Contributing

```bash
python -m venv .venv; source .venv/bin/activate
pip install maturin pytest
maturin develop
pytest
```
