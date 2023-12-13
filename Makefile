clean:
	cargo clean

check:
	cargo check

develop:
	maturin develop --release

test_all:
	cargo test --lib
	pytest

sample:
	python sample.py
