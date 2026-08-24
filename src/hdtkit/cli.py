from __future__ import annotations

from pathlib import Path

import typer
from rich.console import Console

from hdtkit import __version__

app = typer.Typer(
    name="hdtkit",
    help="Convert between RDF Turtle (.ttl) and HDT (.hdt).",
    no_args_is_help=True,
)
console = Console()


def _version_callback(value: bool) -> None:
    if value:
        console.print(f"hdtkit {__version__}")
        raise typer.Exit()


@app.callback()
def main(
    version: bool = typer.Option(
        False,
        "--version",
        callback=_version_callback,
        is_eager=True,
        help="Show the hdtkit version and exit.",
    ),
) -> None:
    """hdtkit: convert RDF Turtle (.ttl) files to HDT and back."""


@app.command("to-hdt")
def to_hdt(
    input_path: Path = typer.Argument(..., exists=True, help="Input .ttl file."),
    output_path: Path = typer.Argument(None, help="Output .hdt file. Defaults to input with .hdt suffix."),
) -> None:
    """Convert a Turtle (.ttl) file to HDT (.hdt)."""
    output_path = output_path or input_path.with_suffix(".hdt")
    console.print(f"[yellow]TODO:[/yellow] convert {input_path} -> {output_path}")


@app.command("to-ttl")
def to_ttl(
    input_path: Path = typer.Argument(..., exists=True, help="Input .hdt file."),
    output_path: Path = typer.Argument(None, help="Output .ttl file. Defaults to input with .ttl suffix."),
) -> None:
    """Convert an HDT (.hdt) file to Turtle (.ttl)."""
    output_path = output_path or input_path.with_suffix(".ttl")
    console.print(f"[yellow]TODO:[/yellow] convert {input_path} -> {output_path}")


if __name__ == "__main__":
    app()
