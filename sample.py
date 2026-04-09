import sys
import time

import numpy as np
from sudoku_rust import __version__, check, solve
print(f"sudoku-rust{__version__}")

if len(sys.argv) == 1:
    print(f"usage: {sys.argv[0]} filename")
    exit()

filepath = sys.argv[1]
x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)
print("Problem:")
print(x)

start = time.perf_counter()

try:
    y = solve(x)
except Exception as e:
    print(f"Error: {e}", file=sys.stderr)
    exit(1)

end = time.perf_counter()
elapsed = end - start

print("Solution:")
print(y)

print(f"elapsed time: {elapsed:.6f}")

if np.all(x == y):
    print("The problem doesn't have any solutions.", file=sys.stderr)
else:
    assert check(y), "Trouble has occurred."
    assert np.all((x == y) == (x != 0)), "Trouble has occurred."
