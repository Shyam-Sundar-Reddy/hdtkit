import pytest

import hdtkit
from hdtkit import hdt_to_ttl, merge_hdt, ttl_to_hdt


def test_exports() -> None:
    assert hdtkit.__version__
    assert callable(ttl_to_hdt)
    assert callable(hdt_to_ttl)
    assert callable(merge_hdt)


def test_ttl_to_hdt_not_implemented_yet() -> None:
    with pytest.raises(NotImplementedError):
        ttl_to_hdt("in.ttl", "out.hdt")


def test_hdt_to_ttl_not_implemented_yet() -> None:
    with pytest.raises(NotImplementedError):
        hdt_to_ttl("in.hdt", "out.ttl")


def test_merge_hdt_requires_at_least_two_inputs() -> None:
    with pytest.raises(ValueError):
        merge_hdt(["only-one.hdt"], "out.hdt")


def test_merge_hdt_not_implemented_yet() -> None:
    with pytest.raises(NotImplementedError):
        merge_hdt(["a.hdt", "b.hdt"], "out.hdt")
