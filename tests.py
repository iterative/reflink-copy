import stat
import os
from os import umask

import pytest

from pathlib import Path
from tempfile import TemporaryDirectory

import reflink_copy


@pytest.fixture(scope="session")
def make_tmp_dir_pytest_cache(request):
    """Create a test directory within cache directory.

    The cache directory by default, is in the root of the repo, where reflink
    may be supported."""
    cache = request.config.cache
    path = cache.mkdir("reflink-copy")

    def inner(name=None):
        tmp_dir = TemporaryDirectory(prefix=name, dir=path)
        request.addfinalizer(tmp_dir.cleanup)
        return Path(tmp_dir.name)

    return inner


@pytest.fixture
def test_dir(make_tmp_dir_pytest_cache):
    return make_tmp_dir_pytest_cache("reflink_test")


@pytest.mark.parametrize("conv", [os.fspath, lambda x: x])
@pytest.mark.parametrize(
    "reflink", [reflink_copy.reflink, reflink_copy.reflink_or_copy]
)
def test_reflink(test_dir, conv, reflink):
    src = test_dir / "source"
    dest = test_dir / "dest"
    src.write_bytes(b"content")

    ret = reflink(conv(src), conv(dest))
    assert ret is None

    assert dest.is_file()
    assert dest.read_bytes() == b"content"

    stat_mode = src.stat().st_mode & (stat.S_IRWXU | stat.S_IRWXG | stat.S_IRWXO)
    assert stat_mode == (0o666 & ~umask(0))


def test_reflink_src_not_existing(test_dir):
    with pytest.raises(FileNotFoundError):
        reflink_copy.reflink(test_dir / "src", test_dir / "dst")


def test_reflink_dst_already_exists(test_dir):
    (test_dir / "src").write_bytes(b"hello")
    (test_dir / "dst").write_bytes(b"hello")
    with pytest.raises(FileExistsError):
        reflink_copy.reflink(test_dir / "src", test_dir / "dst")
