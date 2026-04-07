# sudoku_rust

Python module to solve Sudoku puzzle written in Rust.

For more information on Sudoku, see [Wikipedia](https://en.wikipedia.org/wiki/Sudoku).

## Install make command

- For Windows:

Download the installation kit for your Windows from [here](http://gnuwin32.sourceforge.net/packages/make.htm) and run it.

- For macOS:

```
xcode-select --install
```

- For Debian/Ubuntu:

```
sudo apt install -y make
```

- For CentOS/Fedora:

```
sudo yum install -y make
```

## Install uv - Python package and project manager

- For Windows:

```
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
```

- For macOS, linux, or another Unix-like OS:

```
curl -LsSf https://astral.sh/uv/install.sh | sh
```

## Install Rust

- For Windows:

Download the installation kit for your Windows from [here](https://forge.rust-lang.org/infra/other-installation-methods.eichithi-emueru) and run it.

- For macOS, linux, or another Unix-like OS:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Clone this repository

```
cd [your project directory]
git clone https://github.com/yiwaki/sudoku_rust
```

## Make your environment

- For Windows:

```
cd sudoku_rust
./make_env.ps1
```

- For macOS, linux, or another Unix-like OS:

```
cd sudoku_rust
source ./make_env.sh
```

## Build it and run sample program

```
make develop
make sample
```

## How to make Sudoku puzzle

see the sample data -> data/easy.csv
