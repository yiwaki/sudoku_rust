#![feature(portable_simd)]
use numpy::ndarray::{Array2, ArrayView2, arr2};
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2, PyUntypedArrayMethods};
use pyo3::types::{PyModule, PyModuleMethods};
use pyo3::{Bound, PyResult, Python, pyfunction, pymodule, wrap_pyfunction};

mod sudoku;
use sudoku::is_simd_supported;
use sudoku::matrix::bitmap::{Bitmap, FULL_BIT};
use sudoku::matrix::{MATRIX_SIZE, Matrix};

// implement Into<&ArrayView2<'_, Bitmap>> for Matrix automatically
impl From<&ArrayView2<'_, Bitmap>> for Matrix {
    fn from(x: &ArrayView2<Bitmap>) -> Self {
        let mut it = x.iter();
        [(); MATRIX_SIZE]
            .map(|()| {
                [(); MATRIX_SIZE].map(|()| {
                    let &y = it.next().unwrap();
                    if y == 0 { FULL_BIT } else { 1 << (y - 1) }
                })
            })
            .into()
    }
}

impl Matrix {
    fn into_ndarray(self) -> Array2<Bitmap> {
        arr2(&*self).map(|&x| {
            if x == FULL_BIT {
                0
            } else {
                x.ilog2() as Bitmap + 1
            }
        })
    }
}

#[pyfunction(name = "solve")]
fn wrap_solve<'py>(
    py: Python<'py>,
    arr: PyReadonlyArray2<'py, Bitmap>,
    simd: Option<bool>,
) -> PyResult<Bound<'py, PyArray2<Bitmap>>> {
    let use_simd = simd.unwrap_or(false);
    if arr.shape() != [MATRIX_SIZE, MATRIX_SIZE] {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "Input array must be of shape ({}, {})",
            MATRIX_SIZE, MATRIX_SIZE
        )));
    }

    let Some(solution) = Matrix::from(&arr.as_array()).solve(0, use_simd) else {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "No solution found for the given Sudoku problem.",
        ));
    };
    Ok(solution.into_ndarray().into_pyarray(py))
}

#[pyfunction(name = "check")]
fn wrap_check<'py>(_py: Python<'py>, arr: PyReadonlyArray2<'py, Bitmap>) -> bool {
    if arr.shape() != [MATRIX_SIZE, MATRIX_SIZE] {
        return false;
    }
    Matrix::from(&arr.as_array()).check()
}

#[pymodule]
fn sudoku_rust<'py>(_py: Python<'py>, m: &Bound<'py, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("is_simd_supported", format!("{}", is_simd_supported()))?;
    m.add_function(wrap_pyfunction!(wrap_solve, m)?)?;
    m.add_function(wrap_pyfunction!(wrap_check, m)?)?;
    Ok(())
}
