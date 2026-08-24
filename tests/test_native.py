"""Phase 1: prove the PyO3 native extension round-trips before real conversion
logic depends on it."""

from hdtkit import _native


def test_ping() -> None:
    assert _native.ping() == "pong"
