use numpy::{ndarray::arr2, IntoPyArray, PyArray2, PyReadonlyArray2};
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
        let x = Box::new([(); sudoku::matrix::MATRIX_SIZE].map(|()| {
            [(); sudoku::matrix::MATRIX_SIZE].map(|()| {
                let z = arr.iter().next().unwrap();
                if *z == 0 {
                    sudoku::matrix::bitmap::FULL_BIT
                } else {
                    1 << (*z - 1)
                }
            })
        }));

        let y = sudoku::bruteforce(&x, 0);

        let arr = arr2(&*y).map(|&z| z.ilog2() as sudoku::matrix::bitmap::Bitmap + 1);
        arr.into_pyarray(py)
    }

    Ok(())
}
