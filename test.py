# maturin build -i python --release
# pip install .

import numpy as np
from sudoku_rust import bruteforce

full_bits = 2**9 - 1


def load_csv(filepath):
    problem = np.loadtxt(filepath, delimiter=",").astype(np.uint16)
    print("Puzzle:")
    print(problem)
    working = (lambda x: 1 << (x - 1))(problem)
    working = np.where(working == 0, full_bits, working)
    return working


x = load_csv("data/evil_3.csv")

y = bruteforce(x)
print("Solution:")
print(np.int_(np.log2(y) + 1))
