clean:
	cargo clean
	rm -Force -Recurse .pytest_cache

check:
	cargo clippy

develop:
	maturin develop --release
	pytest

test:
	cargo test --lib

cover:
	cargo llvm-cov --html --open --ignore-filename-regex lib.rs

doc:
	cargo doc

sample:
	python py/sample.py data/evil_3.csv
