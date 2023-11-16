import numpy as np
from sudoku_rust import bruteforce

FULL_BITS = 2**9 - 1


def load_csv(filepath):
    problem = np.loadtxt(filepath, delimiter=",").astype(np.uint16)
    print("Puzzle:")
    print(problem)
    working = (lambda x: 1 << (x - 1))(problem)
    working = np.where(working == 0, FULL_BITS, working)
    return working


x = load_csv("data/evil_3.csv")

y = bruteforce(x)
print("Solution:")
print(np.int_(np.log2(y) + 1))
