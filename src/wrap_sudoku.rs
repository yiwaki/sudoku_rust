use numpy::ndarray::{arr2, Array2, ArrayView2};
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::{pyfunction, pymodule, types::PyModule, wrap_pyfunction, PyResult, Python};

mod sudoku;
use sudoku::matrix::bitmap::{Bitmap, FULL_BIT};
use sudoku::matrix::{Matrix, MATRIX_SIZE};

fn _ndarray_to_matrix(x: &ArrayView2<Bitmap>) -> Matrix {
    let mut it = x.iter();
    Matrix::new([(); MATRIX_SIZE].map(|()| {
        [(); MATRIX_SIZE].map(|()| {
            let z = it.next().unwrap();
            if *z == 0 {
                FULL_BIT
            } else {
                1 << (*z - 1)
            }
        })
    }))
}

fn _matrix_to_ndarray(x: &Matrix) -> Array2<Bitmap> {
    arr2(&**x).map(|z| (*z).ilog2() as Bitmap + 1)
}

#[pyfunction]
fn solve<'py>(py: Python<'py>, arr: PyReadonlyArray2<'py, Bitmap>) -> &'py PyArray2<Bitmap> {
    let x = _ndarray_to_matrix(&arr.as_array());

    let y = x.solve(0);

    _matrix_to_ndarray(&y).into_pyarray(py)
}

#[pymodule]
fn sudoku_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve, m)?)?;

    Ok(())
}
