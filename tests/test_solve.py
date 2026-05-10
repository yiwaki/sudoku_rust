import numpy as np
import pytest

from sudoku_rust import solve, check


def test_solve_easy():
    filepath = "data/easy.csv"
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)
    y = solve(x)
    assert check(y)


def test_solve_hard():
    filepath = "data/hard.csv"
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)
    y = solve(x)
    assert check(y)


def test_solve_evil_1():
    filepath = "data/evil_1.csv"
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)
    y = solve(x)
    assert check(y)


def test_solve_evil_2():
    filepath = "data/evil_2.csv"
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)
    y = solve(x)
    assert check(y)


def test_solve_no_solution():
    filepath = "data/bad_1.csv"
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)

    with pytest.raises(ValueError) as e:
        _ = solve(x)

    assert "No solution found" in str(e.value)


def test_solve_invalid_input():
    filepath = "data/bad_2.csv"
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)

    with pytest.raises(ValueError) as e:
        _ = solve(x)

    assert "Input array must be of shape (9, 9)" in str(e.value)


def test_solve_invalid_dtype():
    filepath = "data/bad_2.csv"
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint32)

    with pytest.raises(TypeError) as e:
        _ = solve(x)

    assert "argument 'problem': 'ndarray' object is not an instance of 'ndarray'" in str(e.value)


def test_check_bad_result():
    filepath = "data/easy.csv"
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)
    assert not check(x)


def test_check_invalid_data():
    filepath = "data/bad_2.csv"
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)

    with pytest.raises(ValueError) as e:
        assert not check(x)

    assert "Input array must be of shape (9, 9)" in str(e.value)
