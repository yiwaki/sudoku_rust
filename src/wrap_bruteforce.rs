use numpy::ndarray::Array;
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray2};
use pyo3::{pymodule, types::PyModule, PyResult, Python};

mod bruteforce;

#[pymodule]
fn sudoku_rust<'py>(_py: Python<'py>, m: &'py PyModule) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "bruteforce")]
    fn bruteforce_py<'py>(
        py: Python<'py>,
        x: PyReadonlyArray2<'py, bruteforce::matrix::bitmap::Bitmap>,
    ) -> &'py PyArray2<bruteforce::matrix::bitmap::Bitmap> {
        let arr = x.as_array();
        let mut x = bruteforce::matrix::alloc_matrix();
        for i in 0..bruteforce::matrix::MATRIX_SIZE {
            for j in 0..bruteforce::matrix::MATRIX_SIZE {
                if arr[(i, j)] == 0 {
                    x[i][j] = bruteforce::matrix::bitmap::FULL_BIT;
                } else {
                    x[i][j] = 1 << (arr[(i, j)] - 1);
                }
            }
        }

        let y = bruteforce::bruteforce(&x, 0);

        let mut ret = Array::zeros((
            bruteforce::matrix::MATRIX_SIZE,
            bruteforce::matrix::MATRIX_SIZE,
        ));
        for i in 0..bruteforce::matrix::MATRIX_SIZE {
            for j in 0..bruteforce::matrix::MATRIX_SIZE {
                ret[(i, j)] = y[i][j].ilog2() as u16 + 1;
            }
        }
        ret.into_pyarray(py)
    }

    Ok(())
}
