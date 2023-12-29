clean:
	cargo clean
	rm -fr .pytest_cache

check:
	cargo clippy

build:
	maturin develop --release
	pytest

test:
	cargo test --lib

cover:
	cargo llvm-cov --html --open --ignore-filename-regex lib.rs

sample:
	python sample.py
