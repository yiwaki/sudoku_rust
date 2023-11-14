clean:
	cargo clean

build:
	maturin build -i python --release
	pip install .

install:
	pip install .

test:
	cargo test --lib --release
