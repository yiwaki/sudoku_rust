use numpy::ndarray::{arr2, Array2, ArrayView2};
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::{pyfunction, pymodule, types::PyModule, wrap_pyfunction, PyResult, Python};

mod sudoku;

fn _ndarray_to_matrix(x: &ArrayView2<sudoku::matrix::bitmap::Bitmap>) -> sudoku::matrix::Matrix {
    let mut it = x.iter();
    sudoku::matrix::Matrix::new([(); sudoku::matrix::MATRIX_SIZE].map(|()| {
        [(); sudoku::matrix::MATRIX_SIZE].map(|()| {
            let z = it.next().unwrap();
            if *z == 0 {
                sudoku::matrix::bitmap::FULL_BIT
            } else {
                1 << (*z - 1)
            }
        })
    }))
}

fn _matrix_to_ndarray(x: &sudoku::matrix::Matrix) -> Array2<sudoku::matrix::bitmap::Bitmap> {
    arr2(&**x).map(|z| (*z).ilog2() as sudoku::matrix::bitmap::Bitmap + 1)
}

#[pyfunction]
fn bruteforce<'py>(
    py: Python<'py>,
    arr: PyReadonlyArray2<'py, sudoku::matrix::bitmap::Bitmap>,
) -> &'py PyArray2<sudoku::matrix::bitmap::Bitmap> {
    let x = _ndarray_to_matrix(&arr.as_array());

    let y = sudoku::bruteforce(&x, 0);

    _matrix_to_ndarray(&y).into_pyarray(py)
}

#[pymodule]
fn sudoku_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(bruteforce, m)?)?;

    Ok(())
}
