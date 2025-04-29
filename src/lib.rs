use numpy::ndarray::{Array2, ArrayView2, arr2};
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::types::{PyModule, PyModuleMethods};
use pyo3::{Bound, PyResult, Python, pyfunction, pymodule, wrap_pyfunction};

mod sudoku;
use sudoku::matrix::bmp::{Bmp, FULL_BIT};
use sudoku::matrix::{MATRIX_SIZE, Matrix};

// implement Into<&ArrayView2<'_, Bitmap>> for Matrix automatically
impl From<&ArrayView2<'_, Bmp>> for Matrix {
    fn from(x: &ArrayView2<Bmp>) -> Self {
        let mut it = x.iter();
        [(); MATRIX_SIZE]
            .map(|()| {
                [(); MATRIX_SIZE].map(|()| {
                    let y = it.next().unwrap_or(&0);
                    if *y == 0 { FULL_BIT } else { 1 << (y - 1) }
                })
            })
            .into()
    }
}

impl Matrix {
    fn into_ndarray(self) -> Array2<Bmp> {
        arr2(&*self).map(|x| {
            if *x == FULL_BIT {
                0
            } else {
                x.ilog2() as Bmp + 1
            }
        })
    }
}

#[pyfunction(name = "solve")]
fn wrap_solve<'py>(py: Python<'py>, arr: PyReadonlyArray2<'py, Bmp>) -> Bound<'py, PyArray2<Bmp>> {
    let problem = Matrix::from(&arr.as_array());
    problem
        .clone()
        .solve(0)
        .unwrap_or(problem)
        .into_ndarray()
        .into_pyarray(py)
}

#[pyfunction(name = "check")]
fn wrap_done<'py>(_py: Python<'py>, arr: PyReadonlyArray2<'py, Bmp>) -> bool {
    Matrix::from(&arr.as_array()).has_done().is_some()
}

#[pymodule]
fn sudoku_rust<'py>(_py: Python<'py>, m: &Bound<'py, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(wrap_solve, m)?)?;
    m.add_function(wrap_pyfunction!(wrap_done, m)?)?;
    Ok(())
}
