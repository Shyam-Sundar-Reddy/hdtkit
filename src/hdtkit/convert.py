"""Public conversion API for hdtkit.

These functions are the stable Python-facing surface. Today they raise
``NotImplementedError`` because the Rust/PyO3 native extension (``hdtkit._native``)
does not exist yet — see ``phase.md`` for the build plan. Once the native extension
lands, each function becomes a thin wrapper around the corresponding
``hdtkit._native`` call, and this signature stays the same.
"""

from __future__ import annotations

from pathlib import Path


def ttl2hdt(
    input_path: str | Path,
    output_path: str | Path,
    *,
    base_uri: str | None = None,
) -> None:
    """Convert a Turtle (``.ttl``) file to HDT (``.hdt``).

    Args:
        input_path: Path to the source ``.ttl`` file.
        output_path: Path to write the resulting ``.hdt`` file.
        base_uri: Optional base URI for resolving relative IRIs while parsing.

    Raises:
        NotImplementedError: Native conversion is not implemented yet (Phase 4).
    """
    raise NotImplementedError(
        "ttl2hdt is not implemented yet — see phase.md, Phase 4 (Rust HDT write path)."
    )


def hdt2ttl(
    input_path: str | Path,
    output_path: str | Path,
) -> None:
    """Convert an HDT (``.hdt``) file to Turtle (``.ttl``).

    Args:
        input_path: Path to the source ``.hdt`` file.
        output_path: Path to write the resulting ``.ttl`` file.

    Raises:
        NotImplementedError: Native conversion is not implemented yet (Phase 3).
    """
    raise NotImplementedError(
        "hdt2ttl is not implemented yet — see phase.md, Phase 3 (Rust HDT read path)."
    )


def hdtcat(
    input_paths: list[str | Path],
    output_path: str | Path,
) -> None:
    """Combine two or more ``.hdt`` files into a single ``.hdt``, de-duplicating triples.

    Args:
        input_paths: Paths of the ``.hdt`` files to merge (2 or more).
        output_path: Path to write the merged ``.hdt`` file.

    Raises:
        ValueError: Fewer than 2 input paths were given.
        NotImplementedError: Native conversion is not implemented yet (Phase 5).
    """
    if len(input_paths) < 2:
        raise ValueError("hdtcat requires at least 2 input .hdt files")
    raise NotImplementedError(
        "hdtcat is not implemented yet — see phase.md, Phase 5 (built on read + write paths)."
    )
