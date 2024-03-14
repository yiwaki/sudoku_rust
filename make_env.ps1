python -m venv .venv
./.venv/Scripts/activate
pip install numpy pytest
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
