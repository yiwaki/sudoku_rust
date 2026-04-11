import numpy.typing as npt

__version__: str
is_simd_supported: bool

def solve(x: npt.NDArray, use_simd: bool) -> npt.NDArray: ...
def check(x: npt.NDArray) -> bool: ...
