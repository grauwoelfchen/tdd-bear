setup: # Prepare dependecies
	@uv run sync
.PHONY: setup

check: # Check types
	@uv run mypy --config-file mypy.ini src test
.PHONY: check

lint: # Check code style with linter
	@uvx ruff check --config ruff.toml --show-fixes
.PHONY: lint

format: # Check code format [alias: fmt]
	@uvx ruff format --config ruff.toml --diff
.PHONY: format

fmt: format
.PHONY: fmt

test: # Run test
	@uv run pytest --config-file pytest.ini test
.PHONY: test

build: # Build the package
	@uv build --out-dir dst
.PHONY: build

clean: # Delete Python cache dirs and files
	@find {src,test} \
		-type f -name '*.py[co]' -delete -o \
		-type d -name __pycache__ -delete
	@rm -fr {mypy,pytest,ruff}_cache
.PHONY: clean

help: # Display this message
	@set -uo pipefail; \
	grep --extended-regexp '^[-_0-9a-z\%\:\\ ]+: # ' \
		$(firstword $(MAKEFILE_LIST)) | \
		sed --expression='s/\( [-_0-9a-z\%\:\\ ]*\) #/ #/' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ": # "}; \
			{printf "\033[38;05;25m%-7s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help

.DEFAULT_GOAL = build
default: build
