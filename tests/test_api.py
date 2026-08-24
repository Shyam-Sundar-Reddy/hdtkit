from pathlib import Path

import pytest

import hdtkit
from hdtkit import hdt2ttl, hdtcat, ttl2hdt


def test_exports() -> None:
    assert hdtkit.__version__
    assert callable(ttl2hdt)
    assert callable(hdt2ttl)
    assert callable(hdtcat)


def test_hdt2ttl_converts_fixture(tmp_path) -> None:
    fixture = Path(__file__).parent / "fixtures" / "snikmeta.hdt"
    out = tmp_path / "snikmeta.ttl"
    hdt2ttl(fixture, out)
    ttl = out.read_text()
    assert ttl
    assert "@prefix" in ttl or "<http" in ttl


def test_ttl2hdt_and_back_round_trips(tmp_path) -> None:
    ttl_in = tmp_path / "in.ttl"
    ttl_in.write_text(
        '@prefix ex: <http://example.org/> .\n'
        'ex:alice ex:knows ex:bob .\n'
        'ex:alice ex:name "Alice"@en .\n'
    )
    hdt_out = tmp_path / "out.hdt"
    ttl2hdt(ttl_in, hdt_out)
    assert hdt_out.exists() and hdt_out.stat().st_size > 0

    ttl_back = tmp_path / "back.ttl"
    hdt2ttl(hdt_out, ttl_back)
    text = ttl_back.read_text()
    assert "alice" in text.lower()
    assert "Alice" in text


def test_hdtcat_requires_at_least_two_inputs() -> None:
    with pytest.raises(ValueError):
        hdtcat(["only-one.hdt"], "out.hdt")


def test_hdtcat_merges_and_dedups(tmp_path) -> None:
    a_ttl = tmp_path / "a.ttl"
    a_ttl.write_text("@prefix ex: <http://example.org/> .\nex:a ex:knows ex:b .\nex:shared ex:p ex:o .\n")
    b_ttl = tmp_path / "b.ttl"
    b_ttl.write_text("@prefix ex: <http://example.org/> .\nex:c ex:knows ex:d .\nex:shared ex:p ex:o .\n")
    a_hdt, b_hdt = tmp_path / "a.hdt", tmp_path / "b.hdt"
    ttl2hdt(a_ttl, a_hdt)
    ttl2hdt(b_ttl, b_hdt)

    out = tmp_path / "combined.hdt"
    hdtcat([a_hdt, b_hdt], out)

    back = tmp_path / "combined.ttl"
    hdt2ttl(out, back)
    text = back.read_text()
    # 3 distinct triples expected: a-knows-b, c-knows-d, shared-p-o (deduped, not 4).
    assert text.count(" .\n") == 3
