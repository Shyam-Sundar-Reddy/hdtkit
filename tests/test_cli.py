from typer.testing import CliRunner

from hdtkit import __version__
from hdtkit.cli import app

runner = CliRunner()


def test_version() -> None:
    result = runner.invoke(app, ["--version"])
    assert result.exit_code == 0
    assert __version__ in result.stdout


def test_help() -> None:
    result = runner.invoke(app, ["--help"])
    assert result.exit_code == 0
    assert "to-hdt" in result.stdout
    assert "to-ttl" in result.stdout
