clean:
	cargo clean

run:
	cargo run

develop:
	maturin develop --release

test:
	cargo test --lib
