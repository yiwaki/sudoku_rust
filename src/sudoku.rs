pub mod matrix;
use matrix::{bmp, bmp::Bmp, bmp::FULL_BIT};

impl matrix::Matrix {
    pub fn has_done(self) -> Option<Self> {
        for block_type in matrix::BLOCK_TYPES {
            for block_no in 0..matrix::MATRIX_SIZE {
                let (row_range, col_range) = matrix::block_range(&block_type, block_no);

                let mut bmp: Bmp = 0;
                for row in row_range {
                    for col in col_range.clone() {
                        bmp |= self[(row, col)];

                        if block_type == matrix::Block::Row && self[(row, col)].count_ones() > 1 {
                            return None;
                        }
                    }
                }

                if bmp != FULL_BIT {
                    return None;
                }
            }
        }
        Some(self)
    }

    fn _check_blocks_by_pivot(&self, block_type: &matrix::Block, pivot: matrix::Address) -> bool {
        let block_no = matrix::addr_to_block_no(block_type, pivot);

        let (row_range, col_range) = matrix::block_range(block_type, block_no);

        let mut bmp: Bmp = 0;
        for row in row_range {
            for col in col_range.clone() {
                bmp |= self[(row, col)];
            }
        }
        bmp == FULL_BIT
    }

    fn _prune_by_pivot(&self, pivot: matrix::Address, target_bit: Bmp) -> Option<Self> {
        let mut x = self.clone();

        x[pivot] = target_bit;

        for block_type in matrix::BLOCK_TYPES {
            let block_no = matrix::addr_to_block_no(&block_type, pivot);
            let (row_range, col_range) = matrix::block_range(&block_type, block_no);

            for row in row_range {
                for col in col_range.clone() {
                    if (row, col) != pivot {
                        x[(row, col)] &= !target_bit; // set off the target bit from bitmap
                    }

                    if x[(row, col)] == 0 {
                        return None;
                    }
                }
            }

            if !x._check_blocks_by_pivot(&block_type, pivot) {
                return None;
            }
        }
        Some(x)
    }

    pub fn solve(self, cell_no: usize) -> Option<Self> {
        if cell_no >= matrix::MATRIX_SIZE * matrix::MATRIX_SIZE {
            return Some(self);
        }

        let pivot = matrix::cell_no_to_addr(cell_no);

        for target_bit in bmp::EachBit::new(self[pivot]) {
            let Some(x) = self._prune_by_pivot(pivot, target_bit) else {
                continue;
            };

            if let Some(x) = x.solve(cell_no + 1) {
                return x.has_done();
            }
        }
        None
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
        solution.clone().has_done().is_some()
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

        assert!(x.clone().has_done().is_none());

        {
            let mut y = y.clone();
            y[(5, 2)] = y[(5, 3)];
            assert!(y.has_done().is_none());
        }

        {
            let mut y = y.clone();
            y[(5, 2)] |= 1;
            assert!(y.has_done().is_none());
        }

        {
            let mut y = y.clone();
            y[(5, 2)] = 0;
            assert!(y.has_done().is_none());
        }

        {
            let mut y = y.clone();
            y[(0, 2)] = y[(0, 3)];
            assert!(!_check_problem_solution(&x, &y));
        }
    }
}
