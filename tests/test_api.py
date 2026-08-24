from pathlib import Path

import pytest

import hdtkit
from hdtkit import hdt2ttl, hdtcat, ttl2hdt


def test_exports() -> None:
    assert hdtkit.__version__
    assert callable(ttl2hdt)
    assert callable(hdt2ttl)
    assert callable(hdtcat)


def test_ttl2hdt_not_implemented_yet() -> None:
    with pytest.raises(NotImplementedError):
        ttl2hdt("in.ttl", "out.hdt")


def test_hdt2ttl_converts_fixture(tmp_path) -> None:
    fixture = Path(__file__).parent / "fixtures" / "snikmeta.hdt"
    out = tmp_path / "snikmeta.ttl"
    hdt2ttl(fixture, out)
    ttl = out.read_text()
    assert ttl
    assert "@prefix" in ttl or "<http" in ttl


def test_hdtcat_requires_at_least_two_inputs() -> None:
    with pytest.raises(ValueError):
        hdtcat(["only-one.hdt"], "out.hdt")


def test_hdtcat_not_implemented_yet() -> None:
    with pytest.raises(NotImplementedError):
        hdtcat(["a.hdt", "b.hdt"], "out.hdt")
