import numpy as np
from sudoku_rust import bruteforce


def load_csv(filepath: str) -> np.ndarray:
    problem = np.loadtxt(filepath, delimiter=",").astype(np.uint16)
    print("Puzzle:")
    print(problem)
    working = (lambda x: 1 << (x - 1))(problem)
    working = np.where(working == 0, 0b111111111, working)
    return working


def display(x: np.ndarray) -> None:
    print("Solution:")
    print(np.int_(np.log2(x) + 1))


x = load_csv("data/evil_3.csv")

y = bruteforce(x)

display(y)
