clean:
	cargo clean

develop:
	maturin develop --release

test:
	cargo test --lib
