# sudoku_rust

Python module to solve Sudoku puzzle written in Rust.

For more information on Sudoku, see [Wikipedia](https://en.wikipedia.org/wiki/Sudoku).

## Install Rust

- For Windows, download the installation kit for your Windows from [here](https://forge.rust-lang.org/infra/other-installation-methods.eichithi-emueru) and run it.
- For macOS, linux, or another Unix-like OS, run the following shell command.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install Python and create virtual environment

Install Python and venv according to your environment, and run the following command to configure your virtual environment of python to your venv directory.

```
python -m venv [your venv directory]
```

## Install Maturin

```
pip install maturin
```

## Compile and install

```
maturin develop --release
```

## Download this kit

```
cd [your project directory]
git clone https://github.com/yiwaki/sudoku_rust
```

## Activate your venv and build it

- For Windows (PowerShell)

```
cd sudoku_rust
[your venv directory]\Scripts\Activate.ps1
make develop
```

- For macOS, Linux, or another Unix-like OS (bash/zsh)

```
cd sudoku_rust
source [your venv directory]/bin/activate
make develop
```

## Run sample program

```
make sample
```

## How to make Sudoku puzzle

see the sample data -> data/easy.csv
