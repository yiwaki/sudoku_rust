use numpy::{ndarray, IntoPyArray, PyArray2, PyReadonlyArray2};
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
        let x = x.as_array();

        // convert NDArray to Rust Array
        let mut iter = x.iter();
        let arr = [(); bruteforce::matrix::MATRIX_SIZE]
            .map(|()| [(); bruteforce::matrix::MATRIX_SIZE].map(|()| *iter.next().unwrap()));

        let y = bruteforce::bruteforce(&arr, 0);

        let y = ndarray::arr2(&y);

        y.into_pyarray(py)
    }

    Ok(())
}
