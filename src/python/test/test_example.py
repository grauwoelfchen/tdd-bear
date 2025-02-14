import shlex
from unittest import mock

from typer.testing import CliRunner

from example import app
import example

runner = CliRunner()


def run(command_string: str) -> str:
    command_list = shlex.split(command_string)
    results = runner.invoke(app, command_list)
    return results.stdout.rstrip()


def test_version() -> None:
    with mock.patch.object(example, "__version__", "0.0.0"):
        assert run("version") == "0.0.0"
