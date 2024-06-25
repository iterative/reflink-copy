from os import PathLike
from typing import Union

StrPath = Union[str, PathLike[str]]

def reflink(src: StrPath, dst: StrPath) -> None: ...
def reflink_or_copy(src: StrPath, dst: StrPath) -> None: ...
