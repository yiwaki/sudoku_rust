use numpy::ndarray::{arr2, Array2, ArrayView2};
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::{pyfunction, pymodule, types::PyModule, wrap_pyfunction, PyResult, Python};

mod sudoku;
use sudoku::matrix::bitmap::{Bitmap, FULL_BIT};
use sudoku::matrix::{Matrix, MATRIX_SIZE};

// implement Into trait for Matrix automatically
impl From<&ArrayView2<'_, Bitmap>> for Matrix {
    fn from(x: &ArrayView2<Bitmap>) -> Self {
        [(); MATRIX_SIZE]
            .map(|()| {
                [(); MATRIX_SIZE].map(|()| {
                    let &y = x.iter().next().unwrap();
                    if y == 0 {
                        FULL_BIT
                    } else {
                        1 << (y - 1)
                    }
                })
            })
            .into()
    }
}

impl Matrix {
    fn to_ndarray(&self) -> Array2<Bitmap> {
        arr2(&**self).map(|&x| x.ilog2() as Bitmap + 1)
    }
}

#[pyfunction(name = "solve")]
fn wrap_solve<'py>(py: Python<'py>, arr: PyReadonlyArray2<'py, Bitmap>) -> &'py PyArray2<Bitmap> {
    Matrix::from(&arr.as_array())
        .solve(0)
        .to_ndarray()
        .into_pyarray(py)
}

#[pymodule]
fn sudoku_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(wrap_solve, m)?)?;

    Ok(())
}
