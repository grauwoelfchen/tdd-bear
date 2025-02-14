# Example

## Requirements

* GNU Make
* uv
  * Python `>= 3.12` e.g. `uv python install 3.12`


## Setup

1. Install uv ([link](https://docs.astral.sh/uv/getting-started/installation/))
2. Run `make setup` in project root directory


## Development

See `make help`

```zsh
% make help
build   Build the package
check   Check types
clean   Delete Python cache dirs and files
format  Check code format [alias: fmt]
help    Display this message
lint    Check code style with linter
setup   Prepare dependecies
test    Run test
```

### How to run the example CLI

```zsh
# via `uv run` or `uvx`
% uv run example --help
```
