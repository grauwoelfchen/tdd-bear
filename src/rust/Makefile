check:
	@cargo check --all -v
.PHONY: check

format:
	@cargo fmt --all -- --check
.PHONY: format

fmt: format
.PHONY: fmt

test:
	@cargo test
.PHONY: test

coverage:
	@cargo build --tests
	@../../bin/check-kcov money_test kcov
.PHONY: coverage

cov: coverage
.PHONY: cov

clean:
	@cargo clean
.PHONY: clean

.DEFAULT_GOAL = test
default: test
