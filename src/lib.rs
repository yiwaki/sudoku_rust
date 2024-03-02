use numpy::ndarray::{arr2, Array2, ArrayView2};
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::{pyfunction, pymodule, types::PyModule, wrap_pyfunction, PyResult, Python};

mod sudoku;
use sudoku::matrix::bitmap::{Bitmap, FULL_BIT};
use sudoku::matrix::{Matrix, MATRIX_SIZE};

// implement Into<&ArrayView2<'_, Bitmap>> for Matrix automatically
impl From<&ArrayView2<'_, Bitmap>> for Matrix {
    fn from(x: &ArrayView2<Bitmap>) -> Self {
        let mut it = x.iter();
        [(); MATRIX_SIZE]
            .map(|()| {
                [(); MATRIX_SIZE].map(|()| {
                    let y = *it.next().unwrap();
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
    fn into_ndarray(self) -> Array2<Bitmap> {
        arr2(&*self).map(|&x| x.ilog2() as Bitmap + 1)
    }
}

#[pyfunction(name = "solve")]
fn wrap_solve<'p>(py: Python<'p>, arr: PyReadonlyArray2<'p, Bitmap>) -> &'p PyArray2<Bitmap> {
    Matrix::from(&arr.as_array())
        .solve(0)
        .unwrap()
        .into_ndarray()
        .into_pyarray(py)
}

#[pyfunction(name = "check")]
fn wrap_done<'p>(_py: Python<'p>, arr: PyReadonlyArray2<'p, Bitmap>) -> bool {
    Matrix::from(&arr.as_array()).has_done().is_some()
}

#[pymodule]
fn sudoku_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(wrap_solve, m)?)?;
    m.add_function(wrap_pyfunction!(wrap_done, m)?)?;
    Ok(())
}
