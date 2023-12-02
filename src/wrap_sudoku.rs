use numpy::ndarray::{arr2, Array2, ArrayView2};
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::{pyfunction, pymodule, types::PyModule, wrap_pyfunction, PyResult, Python};

mod sudoku;
use sudoku::matrix::bitmap::{Bitmap, FULL_BIT};
use sudoku::matrix::{Matrix, MATRIX_SIZE};

impl Matrix {
    fn ndarray_to_matrix(x: &ArrayView2<Bitmap>) -> Self {
        Matrix::new([(); MATRIX_SIZE].map(|()| {
            [(); MATRIX_SIZE].map(|()| {
                let z = x.iter().next().unwrap();
                if *z == 0 {
                    FULL_BIT
                } else {
                    1 << (*z - 1)
                }
            })
        }))
    }

    fn matrix_to_ndarray(&self) -> Array2<Bitmap> {
        arr2(&**self).map(|z| (*z).ilog2() as Bitmap + 1)
    }
}

#[pyfunction]
fn solve<'py>(py: Python<'py>, arr: PyReadonlyArray2<'py, Bitmap>) -> &'py PyArray2<Bitmap> {
    Matrix::ndarray_to_matrix(&arr.as_array())
        .solve(0)
        .matrix_to_ndarray()
        .into_pyarray(py)
}

#[pymodule]
fn sudoku_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve, m)?)?;

    Ok(())
}
