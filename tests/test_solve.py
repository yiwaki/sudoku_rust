import numpy as np

from sudoku_rust import solve, check


def test_solve():
    filepath = "data/evil_3.csv"
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)

    y = solve(x)

    assert check(y)
    assert np.all((x == y) == (x != 0))
