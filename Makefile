clean:
	cargo clean

build:
	maturin build -i python --release

install:
	pip install .

develop:
	maturin build -i python --release
	pip install .

test:
	cargo test --lib --release
