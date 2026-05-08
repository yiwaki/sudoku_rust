pub mod matrix;
use matrix::{BLOCK_TYPES, NUM_OF_CELLS, bitmap};

impl matrix::Matrix {
    fn _prune_by_pivot(&self, pivot: matrix::Address, target_bit: bitmap::Bitmap) -> Option<Self> {
        //! Prune the matrix by setting off the target bit from the bitmap of cells
        //! in the same row, column and square as the pivot cell.
        let mut x = self.clone();
        x[pivot] = target_bit;

        BLOCK_TYPES.iter().try_for_each(|block_type| {
            let block_no = matrix::addr_to_block_no(block_type, pivot);
            let (row_range, col_range) = matrix::block_range(block_type, block_no);

            row_range
                .flat_map(|row| col_range.clone().map(move |col| (row, col)))
                .filter(|&addr| addr != pivot)
                .try_for_each(|addr| {
                    let cell = &mut x[addr];
                    *cell &= !target_bit;

                    (*cell != 0).then_some(())
                })
        })?;

        Some(x)
    }

    pub fn solve(self, cell_no: usize) -> Option<Self> {
        //! cell_no is the index of the cell to be solved, in row-major order
        if cell_no >= NUM_OF_CELLS {
            return Some(self);
        }

        let pivot = matrix::cell_no_to_addr(cell_no);

        bitmap::EachBit::new(self[pivot]).find_map(|target_bit| {
            self._prune_by_pivot(pivot, target_bit)
                .and_then(|pruned| pruned.solve(cell_no + 1))
        })
    }

    pub fn check(&self) -> bool {
        //! Check if the matrix is a valid solution of Sudoku.
        BLOCK_TYPES.iter().all(|block_type| {
            (0..matrix::MATRIX_SIZE).all(|block_no| {
                let (row_range, col_range) = matrix::block_range(block_type, block_no);
                let cells = row_range.flat_map(|row| col_range.clone().map(move |col| (row, col)));
                if *block_type == matrix::Block::Row
                    && cells.clone().any(|addr| self[addr].count_ones() > 1)
                {
                    return false;
                }

                cells.fold(0, |acc, addr| acc | self[addr]) == matrix::bitmap::FULL_BIT
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn _check_problem_solution(problem: &matrix::Matrix, solution: &matrix::Matrix) -> bool {
        (0..matrix::NUM_OF_CELLS).all(|cell_no| {
            let address = matrix::cell_no_to_addr(cell_no);
            problem[address] == bitmap::FULL_BIT || problem[address] == solution[address]
        }) && solution.check()
    }

    #[test]
    fn solve_test() {
        let x: matrix::Matrix = [
            [0, 0, 4, 0, 5, 0, 0, 0, 1],
            [0, 0, 6, 0, 0, 0, 0, 3, 0],
            [5, 3, 0, 7, 0, 0, 0, 0, 8],
            [1, 2, 0, 0, 6, 0, 0, 8, 0],
            [0, 0, 3, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 9, 0, 0, 7],
            [4, 0, 0, 0, 0, 0, 0, 0, 0],
            [8, 5, 0, 0, 1, 0, 0, 2, 0],
            [0, 0, 0, 6, 0, 0, 1, 0, 0],
        ]
        .map(|row| {
            row.map(|col| {
                if col == 0 {
                    bitmap::FULL_BIT
                } else {
                    1 << (col - 1)
                }
            })
        })
        .into();

        println!("Problem:");
        println!("{}", x);

        let start = Utc::now().time();

        let y = x.clone().solve(0).unwrap();

        println!("Solution:");
        println!("{}", y);
        let end = Utc::now().time();

        let dt = (end - start).num_microseconds().unwrap() as f64 * 1E-6;
        println!("elapsed time: {:0.6}", dt);

        assert!(_check_problem_solution(&x, &y));

        assert!(!x.check());

        {
            let mut y = y.clone();
            y[(5, 2)] = y[(5, 3)];
            assert!(!y.check());
        }

        {
            let mut y = y.clone();
            y[(5, 2)] |= 1;
            assert!(!y.check());
        }

        {
            let mut y = y.clone();
            y[(5, 2)] = 0;
            assert!(!y.check());
        }

        {
            let mut y = y.clone();
            y[(0, 2)] = y[(0, 3)];
            assert!(!_check_problem_solution(&x, &y));
        }
    }
}
