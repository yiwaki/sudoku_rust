# sudoku-rust

Python module to solve Sudoku puzzle written in Rust.

## Installation

```
pip install sudoku-rust
```

## Sample Python Code (sample.py)

```Python:sample.py
import sys
import time

import numpy as np
from sudoku_rust import __version__, check, solve
print(f"sudoku-rust{__version__}")

if len(sys.argv) == 1:
    print(f"usage: {sys.argv[0]} filename")
    exit(1)

try:
    filepath = sys.argv[1]
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)
    print("Problem:")
    print(x)

    start = time.perf_counter()

    y = solve(x)

    end = time.perf_counter()
    elapsed = end - start

    print("Solution:")
    print(y)

    print(f"elapsed time: {elapsed:.6f}")

except Exception as e:
    print(f"Error: {e}", file=sys.stderr)
    exit(1)

assert np.all((x == y) == (x != 0)), "The solution is not consistent with the input problem."
assert check(y), "The solution is invalid."
```

## Sample Data (easy.csv)

```easy.csv
1,3,0,7,9,0,0,4,5
0,0,5,0,0,0,0,0,6
2,0,0,0,8,0,9,0,0
3,1,0,4,6,5,8,0,0
4,0,0,0,0,0,0,0,2
0,0,7,9,3,2,0,1,4
0,0,3,0,7,0,0,0,8
7,0,0,0,0,0,3,0,0
8,9,0,0,5,4,0,2,1
```
