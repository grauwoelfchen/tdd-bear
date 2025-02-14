"""Example CLI."""

import typer

import example

app = typer.Typer(add_completion=False)


@app.command()
def version() -> None:
    """Print version of example application."""
    print(example.__version__)


@app.command()
def hello() -> None:
    """Print hello message."""
    print("Hello from example!")
