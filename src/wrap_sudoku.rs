use numpy::{ndarray::Array, IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::{pymodule, types::PyModule, PyResult, Python};

mod sudoku;

#[pymodule]
fn sudoku_rust<'py>(_py: Python<'py>, m: &'py PyModule) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "bruteforce")]
    fn bruteforce_py<'py>(
        py: Python<'py>,
        arr: PyReadonlyArray2<'py, sudoku::matrix::bitmap::Bitmap>,
    ) -> &'py PyArray2<sudoku::matrix::bitmap::Bitmap> {
        let arr = arr.as_array();

        let mut x = sudoku::matrix::alloc_matrix();
        for i in 0..sudoku::matrix::MATRIX_SIZE {
            for j in 0..sudoku::matrix::MATRIX_SIZE {
                if arr[(i, j)] == 0 {
                    x[i][j] = sudoku::matrix::bitmap::FULL_BIT;
                } else {
                    x[i][j] = 1 << (arr[(i, j)] - 1);
                }
            }
        }

        let y = sudoku::bruteforce(&x, 0);

        let mut arr = Array::zeros((sudoku::matrix::MATRIX_SIZE, sudoku::matrix::MATRIX_SIZE));
        for i in 0..sudoku::matrix::MATRIX_SIZE {
            for j in 0..sudoku::matrix::MATRIX_SIZE {
                arr[(i, j)] = y[i][j].ilog2() as sudoku::matrix::bitmap::Bitmap + 1;
            }
        }
        arr.into_pyarray(py)
    }

    Ok(())
}
