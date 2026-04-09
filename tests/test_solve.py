import numpy as np
import pytest

from sudoku_rust import solve, check


def test_solve_success():
    filepath = "data/evil_3.csv"
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)

    y = solve(x)

    assert check(y)


def test_solve_fail_1():
    filepath = "data/bad.csv"
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)

    with pytest.raises(ValueError) as e:
        y = solve(x)

    assert str(e.value) == "No solution found for the given Sudoku problem."


def test_solve_fail_2():
    filepath = "data/badbad.csv"
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)

    with pytest.raises(ValueError) as e:
        y = solve(x)

    assert str(e.value) == "Input array must be of shape (9, 9)"