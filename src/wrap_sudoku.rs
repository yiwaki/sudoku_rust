use numpy::{ndarray, IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::{pymodule, types::PyModule, PyResult, Python};

mod sudoku;

fn _array_to_matrix(
    x: &ndarray::ArrayView2<sudoku::matrix::bitmap::Bitmap>,
) -> sudoku::matrix::Matrix {
    Box::new([(); sudoku::matrix::MATRIX_SIZE].map(|()| {
        [(); sudoku::matrix::MATRIX_SIZE].map(|()| {
            let z = x.iter().next().unwrap();
            if *z == 0 {
                sudoku::matrix::bitmap::FULL_BIT
            } else {
                1 << (*z - 1)
            }
        })
    }))
}

fn _matrix_to_array(x: &sudoku::matrix::Matrix) -> ndarray::Array2<sudoku::matrix::bitmap::Bitmap> {
    ndarray::arr2(&**x).map(|z| (*z).ilog2() as sudoku::matrix::bitmap::Bitmap + 1)
}

#[pymodule]
fn sudoku_rust<'py>(_py: Python<'py>, m: &'py PyModule) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "bruteforce")]
    fn bruteforce_py<'py>(
        py: Python<'py>,
        arr: PyReadonlyArray2<'py, sudoku::matrix::bitmap::Bitmap>,
    ) -> &'py PyArray2<sudoku::matrix::bitmap::Bitmap> {
        let x = _array_to_matrix(&arr.as_array());

        let y = sudoku::bruteforce(&x, 0);

        _matrix_to_array(&y).into_pyarray(py)
    }

    Ok(())
}
