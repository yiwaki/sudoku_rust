import numpy.typing as npt

def solve(x: npt.NDArray) -> npt.NDArray:
    """数独を解く

    Args:
        x (NDArray): 数独の問題

    Returns:
        NDArray: 回答
    """
    ...

def check(x: npt.NDArray) -> bool:
    """数独の回答が正しいかチェックする

    Args:
        x (NDArray): チェックする回答

    Returns:
        bool: xが正解ならtrue, そうでなければfalse
    """
    ...
