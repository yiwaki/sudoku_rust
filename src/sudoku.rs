pub mod matrix;
use matrix::bitmap;

impl matrix::Matrix {
    fn _done(&self) -> bool {
        static mut DONE: bool = false;
        unsafe {
            if DONE {
                return true;
            }
        }

        for block_type in matrix::BLOCK_TYPES {
            for block_no in 0..matrix::MATRIX_SIZE {
                let (row_range, col_range) = matrix::block_range(&block_type, block_no);

                let mut bitmap: bitmap::Bitmap = 0;
                for row in row_range {
                    for col in col_range {
                        if bitmap::popcount(self[(row, col)]) > 1 {
                            return false;
                        }

                        bitmap |= self[(row, col)];
                    }
                }

                if bitmap != bitmap::FULL_BIT {
                    return false;
                }
            }
        }

        unsafe {
            DONE = true;
        }
        true
    }

    fn _check_blocks_by_pivot(self, pivot: matrix::Address) -> Option<Self> {
        for block_type in matrix::BLOCK_TYPES {
            let block_no = matrix::addr_to_block_no(&block_type, pivot);
            let (row_range, col_range) = matrix::block_range(&block_type, block_no);

            let mut bitmap: bitmap::Bitmap = 0;
            for row in row_range {
                for col in col_range {
                    bitmap |= self[(row, col)];
                }
            }

            if bitmap != bitmap::FULL_BIT {
                return None;
            }
        }
        Some(self)
    }

    fn _pruned_by_pivot(&self, pivot: matrix::Address, target_bit: bitmap::Bitmap) -> Option<Self> {
        let mut x = self.clone();

        for block_type in matrix::BLOCK_TYPES {
            let block_no = matrix::addr_to_block_no(&block_type, pivot);

            let (row_range, col_range) = matrix::block_range(&block_type, block_no);

            for row in row_range {
                for col in col_range {
                    if (row, col) == pivot {
                        x[(row, col)] = target_bit;
                        continue;
                    }

                    x[(row, col)] &= !target_bit;

                    if x[(row, col)] == 0 {
                        return None;
                    }
                }
            }
        }

        x._check_blocks_by_pivot(pivot)
    }

    pub fn solve(&self, cell_no: usize) -> Self {
        if cell_no >= matrix::MATRIX_SIZE * matrix::MATRIX_SIZE {
            return self.clone();
        }

        let mut x = matrix::Matrix::new();

        let pivot = matrix::cell_no_to_addr(cell_no);

        for target_bit in bitmap::EachBit::from(self[pivot]) {
            x = match self._pruned_by_pivot(pivot, target_bit) {
                Some(y) => y,
                None => {
                    continue;
                }
            };

            x = x.solve(cell_no + 1);

            if x._done() {
                return x;
            };
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _check_problem(problem: &matrix::Matrix, solution: &matrix::Matrix) -> bool {
        for row in 0..matrix::MATRIX_SIZE {
            for col in 0..matrix::MATRIX_SIZE {
                if problem[(row, col)] != bitmap::FULL_BIT
                    && problem[(row, col)] != solution[(row, col)]
                {
                    return false;
                }
            }
        }
        solution._done()
    }

    #[test]
    fn bruteforce_test() {
        let x = matrix::Matrix::from([
            [511, 511, 8, 511, 16, 511, 511, 511, 1],
            [511, 511, 32, 511, 511, 511, 511, 4, 511],
            [16, 4, 511, 64, 511, 511, 511, 511, 128],
            [1, 2, 511, 511, 32, 511, 511, 128, 511],
            [511, 511, 4, 511, 511, 511, 511, 511, 511],
            [511, 511, 511, 511, 511, 256, 511, 511, 64],
            [8, 511, 511, 511, 511, 511, 511, 511, 511],
            [128, 16, 511, 511, 1, 511, 511, 2, 511],
            [511, 511, 511, 32, 511, 511, 1, 511, 511],
        ]);
        println!("Puzzle:");
        println!("{}", x);

        let y = x.solve(0);

        println!("Solution:");
        println!("{}", y);
        assert!(_check_problem(&x, &y));
    }
}
