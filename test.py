import time
import numpy as np
from sudoku_rust import bruteforce

filepath = "data/evil_3.csv"
x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)
print("Puzzle:")
print(x)

start = time.process_time()
y = bruteforce(x)
end = time.process_time()
elapsed = end - start

print("Solution:")
print(y)

print(f"elapsed time: {elapsed:.6f}")
