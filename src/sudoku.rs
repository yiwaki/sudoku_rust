pub mod matrix;
use matrix::{PivotBlockAddrIter, bitmap, bitmap::Bitmap, bitmap::FULL_BIT};
use std::simd::u16x32;

pub fn is_simd_supported() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        is_x86_feature_detected!("avx2")
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        true
    }
}

impl matrix::Matrix {
    fn _prune_by_pivot_scalar(
        &self,
        pivot: matrix::Address,
        target_bit: Bitmap,
    ) -> Result<Self, ()> {
        //! Prune the matrix by setting off the target bit from the bitmap of cells
        //! in the same row, column and square as the pivot cell.
        let mut x = self.clone();

        x[pivot] = target_bit;

        for block_type in matrix::BLOCK_TYPES {
            let block_no = matrix::addr_to_block_no(&block_type, pivot);
            let (row_range, col_range) = matrix::block_range(&block_type, block_no);

            for row in row_range {
                for col in col_range.clone() {
                    if (row, col) != pivot {
                        x[(row, col)] &= !target_bit; // set off the target bit to bitmap
                    }

                    if x[(row, col)] == 0 {
                        return Err(());
                    }
                }
            }
        }
        Ok(x)
    }

    fn _prune_by_pivot_simd(&self, pivot: matrix::Address, target_bit: Bitmap) -> Result<Self, ()> {
        let mut x = self.clone();

        x[pivot] = target_bit;
        let mut bitmap_row = u16x32::splat(0);

        let iter = PivotBlockAddrIter::new(pivot);
        for (i, addr) in iter.enumerate() {
            bitmap_row[i] = self[addr];
        }

        bitmap_row &= u16x32::splat(!target_bit);

        let iter = PivotBlockAddrIter::new(pivot);
        for (i, addr) in iter.enumerate() {
            if bitmap_row[i] == 0 {
                return Err(());
            }
            x[addr] = bitmap_row[i];
        }

        Ok(x)
    }

    fn _prune_by_pivot(
        &self,
        pivot: matrix::Address,
        target_bit: Bitmap,
        use_simd: bool,
    ) -> Result<Self, ()> {
        if use_simd {
            self._prune_by_pivot_simd(pivot, target_bit)
        } else {
            self._prune_by_pivot_scalar(pivot, target_bit)
        }
    }

    pub fn solve(self, cell_no: usize, use_simd: bool) -> Result<Self, ()> {
        //! cell_no is the index of the cell to be solved, in row-major order
        if cell_no >= matrix::NUM_OF_CELLS {
            return Ok(self);
        }

        let pivot = matrix::cell_no_to_addr(cell_no);

        for target_bit in bitmap::EachBit::new(self[pivot]) {
            let Ok(x) = self._prune_by_pivot(pivot, target_bit, use_simd) else {
                continue;
            };

            if let Ok(x) = x.solve(cell_no + 1, use_simd) {
                return Ok(x);
            }
        }
        Err(())
    }

    fn _check_blocks_by_pivot(&self, block_type: &matrix::Block, pivot: matrix::Address) -> bool {
        //! Check if the block of the given type that contains the pivot cell is valid.
        // let mut iter = matrix::PivotBlockAddrIter::new(pivot);

        let block_no = matrix::addr_to_block_no(block_type, pivot);

        let (row_range, col_range) = matrix::block_range(block_type, block_no);

        let mut bmp: Bitmap = 0;
        for row in row_range {
            for col in col_range.clone() {
                bmp |= self[(row, col)];
            }
        }
        bmp == FULL_BIT
    }

    pub fn check(&self) -> bool {
        //! Check if the matrix is a valid solution of Sudoku.
        for block_type in matrix::BLOCK_TYPES {
            for block_no in 0..matrix::MATRIX_SIZE {
                let (row_range, col_range) = matrix::block_range(&block_type, block_no);

                let mut bmp: Bitmap = 0;
                for row in row_range {
                    for col in col_range.clone() {
                        bmp |= self[(row, col)];

                        if block_type == matrix::Block::Row && self[(row, col)].count_ones() > 1 {
                            return false;
                        }
                    }
                }

                if bmp != FULL_BIT {
                    return false;
                }
            }
        }
        true
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
    fn solve_scalar_test() {
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

        let y = x.clone().solve(0, false).unwrap();

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

    #[test]
    fn solve_simd_test() {
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

        let y = x.clone().solve(0, true).unwrap();

        println!("Solution:");
        println!("{}", y);
        let end = Utc::now().time();

        let dt = (end - start).num_microseconds().unwrap() as f64 * 1E-6;
        println!("elapsed time: {:0.6}", dt);

        assert!(_check_problem_solution(&x, &y));

        assert!(!x.check());
    }
}
