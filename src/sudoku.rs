pub mod matrix;
use matrix::{Address, BLOCK_TYPES, Block, bitmap, bitmap::Bitmap, bitmap::FULL_BIT, block_range};
use std::cmp::Ordering::{Equal, Greater, Less};

impl matrix::Matrix {
    fn _prune_by_pivot(&self, pivot: Address, target_bit: Bitmap) -> Option<Self> {
        //! Prune the matrix by setting off the target bit from the bitmap of cells
        //! in the same row, column and square as the pivot cell.
        let mut x = self.clone();
        x[pivot] = target_bit;

        for block_type in &BLOCK_TYPES {
            let block_no = matrix::addr_to_block_no(block_type, pivot);
            let (row_range, col_range) = block_range(block_type, block_no);

            row_range
                .flat_map(|row| col_range.clone().map(move |col| (row, col)))
                .filter(|&addr| addr != pivot)
                .try_for_each(|addr| {
                    let cell = &mut x[addr];
                    *cell &= !target_bit;

                    (*cell != 0).then_some(())
                })?;
        }
        Some(x)
    }

    pub fn solve(self, cell_no: usize) -> Option<Self> {
        //! cell_no is the index of the cell to be solved, in row-major order
        match cell_no.cmp(&matrix::NUM_OF_CELLS) {
            Equal => return Some(self),
            Less => (),
            Greater => {
                panic!("cell_no must be less than or equal to NUM_OF_CELLS")
            }
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
                let (row_range, col_range) = block_range(block_type, block_no);

                let cells = row_range.flat_map(|r| col_range.clone().map(move |c| (r, c)));

                if *block_type == Block::Row
                    && cells.clone().any(|addr| self[addr].count_ones() >= 2)
                {
                    return false;
                }

                cells.fold(0, |acc, addr| acc | self[addr]) == FULL_BIT
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn _check_problem_solution(problem: &matrix::Matrix, solution: &matrix::Matrix) -> bool {
        for row in 0..matrix::MATRIX_SIZE {
            for col in 0..matrix::MATRIX_SIZE {
                if problem[(row, col)] != FULL_BIT && problem[(row, col)] != solution[(row, col)] {
                    return false;
                }
            }
        }
        solution.check()
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
        .map(|r| r.map(|c| if c == 0 { FULL_BIT } else { 1 << (c - 1) }))
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
