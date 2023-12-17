clean:
	cargo clean

check:
	cargo clippy

develop:
	maturin develop --release

test:
	cargo test --lib
	pytest

sample:
	python sample.py
