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
        let arr = arr.as_array().map(|x| {
            if *x == 0 {
                sudoku::matrix::bitmap::FULL_BIT
            } else {
                (1 << (*x - 1)) as sudoku::matrix::bitmap::Bitmap
            }
        });
        let x = [(); sudoku::matrix::MATRIX_SIZE]
            .map(|()| [(); sudoku::matrix::MATRIX_SIZE].map(|()| *arr.iter().next().unwrap()));
        let x = Box::new(x);

        let y = sudoku::bruteforce(&x, 0);

        // let arr = Array2(y);
        // let arr = arr.map(|x| x)
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
