import numpy as np
from sudoku_rust import bruteforce


filepath = "data/evil_3.csv"
x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)
print("Puzzle:")
print(x)

y = bruteforce(x)

print("Solution:")
print(y)
