# sudoku_rust

Python module to solve Sudoku puzzle written in Rust.

For more information about Sudoku, see [Wikipedia](https://en.wikipedia.org/wiki/Sudoku).

## Install make command

- For Windows:

```PowerShell
PS> winget install GnuWin32.Make
```

- For macOS:

```macOS Terminal
$ xcode-select --install
```

- For Debian/Ubuntu:

```Debian Terminal
$ sudo apt install -y make
```

- For CentOS/Fedora:

```CentOS Terminal
$ sudo yum install -y make
```

## Install uv - Python package and project manager

- For Windows:

```PowerShell
PS> powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
```

- For macOS, linux, or other Unix-like OS:

```macOS, Linux, or other Unix-like OS Terminal
$ curl -LsSf https://astral.sh/uv/install.sh | sh
```

## Install Rust

- For Windows:

```PowerShell
PS> winget Rustlang.Rustup
```

- For macOS, linux, or other Unix-like OS:

```macOS, linux, or other Unix-like OS terminal
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Clone this repository

- For Any OS:
 
```Any OS terminal
cd [your projects directory]
git clone https://github.com/yiwaki/sudoku_rust
```

## Make your environment

- For Windows:

```PowerShell
PS> cd sudoku_rust
PS> ./make_env.ps1
```

- For macOS, linux, or other Unix-like OS:

```macOS, linux, or other Unix-like OS terminal
$ cd sudoku_rust
$ source ./make_env.sh
```

## Build it and run sample problem

- For Any OS:

```Any OS terminal
make develop
make sample
```

## How to make Sudoku puzzle

see the sample data -> data/easy.csv
