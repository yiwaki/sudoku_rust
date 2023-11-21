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
        // convert NDArray (Python) to Vector (Rust)
        let x = x.as_array();
        let mut iter = x.iter();
        // let arr = [(); bruteforce::matrix::MATRIX_SIZE]
        //     .map(|()| [(); bruteforce::matrix::MATRIX_SIZE].map(|()| *iter.next().unwrap()));
        let mut v = vec![vec![0; bruteforce::matrix::MATRIX_SIZE]; bruteforce::matrix::MATRIX_SIZE];
        for i in 0..bruteforce::matrix::MATRIX_SIZE {
            for j in 0..bruteforce::matrix::MATRIX_SIZE {
                let tmp = *iter.next().unwrap();
                if tmp == 0 {
                    v[i][j] = bruteforce::matrix::bitmap::FULL_BIT;
                } else {
                    v[i][j] = 1 << (tmp - 1);
                }
            }
        }

        let y = bruteforce::bruteforce(&v, 0);

        // convert Vector (Rust) to NDArray (Python)
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
