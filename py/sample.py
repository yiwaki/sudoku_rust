import sys
import time

import numpy as np

from sudoku_rust import __version__, check, solve

print(f"sudoku-rust ver.{__version__}")


def print_sudoku(board):
    horizontal_line = "+-----+-----+-----+"

    for i in range(9):
        # 3行ごとに横線を表示
        if i % 3 == 0:
            print(horizontal_line)

        row_str = ""
        for j in range(9):
            # 3列ごとに縦線を表示
            if j % 3 == 0:
                row_str += "|"
            else:
                row_str += " "

            # ブロック内の位置に応じてスペースを調整（例に合わせた配置）
            if j == 0:
                row_str += f"{board[i, j]}"
            elif j % 3 == 2:
                row_str += f"{board[i, j]}"
            else:
                row_str += f"{board[i, j]}"

        row_str += "|"
        print(row_str)

    print(horizontal_line)


if len(sys.argv) == 1:
    print(f"usage: {sys.argv[0]} filename")
    exit(1)

try:
    filepath = sys.argv[1]
    x = np.loadtxt(filepath, delimiter=",").astype(np.uint16)
    print("Problem:")
    print_sudoku(x)

    start = time.perf_counter()

    y = solve(x)

    end = time.perf_counter()
    elapsed = end - start

    print("Solution:")
    print_sudoku(y)

    print(f"elapsed time: {elapsed:.6f}")

except Exception as e:
    print(f"Error: {e}", file=sys.stderr)
    exit(1)

assert np.all((x == y) == (x != 0)), (
    "The solution is not consistent with the input problem."
)
assert check(y), "The solution is invalid."
