uv python install 3.12
uv python pin 3.12
uv venv
source ./.venv/bin/activate
uv tool install maturin
uv sync
cargo install cargo-llvm-cov
rustup component add llvm-tools-preview
