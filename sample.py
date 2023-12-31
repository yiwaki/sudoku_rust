import time

import numpy as np

from sudoku_rust import check, solve

filepath = "data/evil_3.csv"
x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)
print("Puzzle:")
print(x)

start = time.perf_counter()
y = solve(x)
end = time.perf_counter()
elapsed = end - start

print("Solution:")
print(y)

print(f"elapsed time: {elapsed:.6f}")

assert check(y)
assert np.all((x == y) == (x != 0))
