# sudoku_rust

Python module to solve Sudoku puzzle written in Rust.

For more information on Sudoku, see [Wikipedia](https://en.wikipedia.org/wiki/Sudoku).

## Install make command

For Windows, download the installation kit for your Windows from [here](http://gnuwin32.sourceforge.net/packages/make.htm) and run it.

For macOS:

```
xcode-select --install
```

For Debian/Ubuntu:

```
sudo apt install -y make
```

For CentOS/Fedora:

```
sudo yum install make -y
```

## Install Python and venv

Install Python and venv according to your environment.

## Install Rust

For Windows, download the installation kit for your Windows from [here](https://forge.rust-lang.org/infra/other-installation-methods.eichithi-emueru) and run it.

For macOS, linux, or another Unix-like OS, run the following shell command.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Install Maturin

```
pip install maturin
```

## Download this kit

```
cd [your project directory]
git clone https://github.com/yiwaki/sudoku_rust
```

## Make your environment

```
cd sudoku_rust
./make_env
```

## Activate your venv

- For Windows

```
./venv/Scripts/activate
```

- For maxOS, Linux, or another Unix-like OS

```
./venv/bin/activate
```

## Build it and run sample program

```
make develop
make sample
```

## How to make Sudoku puzzle

see the sample data -> data/easy.csv
