clean:
	cargo clean

check:
	cargo clippy

develop:
	maturin develop --release
	pytest

test:
	cargo test --lib

sample:
	python sample.py
