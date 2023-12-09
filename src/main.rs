use chrono::Utc;

mod sudoku;
use sudoku::matrix::bitmap::{Bitmap, FULL_BIT};
use sudoku::matrix::{Matrix, MatrixBuffer};

fn disp_matrix(x: &MatrixBuffer) {
    for row in x.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}

fn main() {
    let problem = [
        [1, 3, 0, 7, 9, 0, 0, 4, 5],
        [0, 0, 5, 0, 0, 0, 0, 0, 6],
        [2, 0, 0, 0, 8, 0, 9, 0, 0],
        [3, 1, 0, 4, 6, 5, 8, 0, 0],
        [4, 0, 0, 0, 0, 0, 0, 0, 2],
        [0, 0, 7, 9, 3, 2, 0, 1, 4],
        [0, 0, 3, 0, 7, 0, 0, 0, 8],
        [7, 0, 0, 0, 0, 0, 3, 0, 0],
        [8, 9, 0, 0, 5, 4, 0, 2, 1],
    ];

    let x: Matrix = problem
        .map(|y| y.map(|z| if z == 0 { FULL_BIT } else { 1 << (z - 1) }))
        .into();

    println!("Problem:");
    disp_matrix(&problem);

    let start = Utc::now().time();

    let solution = x.solve(0);

    let end = Utc::now().time();

    println!("Solution:");
    disp_matrix(&solution.map(|y| y.map(|z| z.ilog2() as Bitmap + 1)));

    let dt = (end - start).num_microseconds().unwrap() as f64 * 1E-6;
    println!("elapsed time: {:0.6}", dt);
}
