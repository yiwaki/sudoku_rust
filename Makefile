clean:
	cargo clean
	rm -Force -Recurse .pytest_cache

check:
	cargo clippy

develop:
	maturin develop --release
	uv run pytest

test:
	cargo test --lib

cover:
	cargo llvm-cov --html --open --ignore-filename-regex lib.rs

doc:
	cargo doc

sample:
	uv run sample.py data/evil_3.csv
