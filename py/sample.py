import sys
import time

import numpy as np
from sudoku_rust import __version__, is_simd_supported, check, solve
print(f"sudoku-rust{__version__}")
print(f"SIMD supported: {is_simd_supported}")

simd = None
if len(sys.argv) == 1:
    print(f"usage: {sys.argv[0]} filename")
    exit(1)

elif len(sys.argv) == 2:
    simd = False

else:
    simd_arg = sys.argv[2].lower()
    if simd_arg in ("true", "1", "yes"):
        simd = True
    elif simd_arg in ("false", "0", "no"):
        simd = False
    else:
        print(f"Invalid SIMD argument: {sys.argv[2]}. Use 'true' or 'false'.", file=sys.stderr)
        exit(1)

try:
    filepath = sys.argv[1]
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)
    print("Problem:")
    print(x)

    start = time.perf_counter()

    y = solve(x, simd)

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
