pub mod matrix;
use matrix::bitmap;

impl matrix::Matrix {
    fn _done(&self) -> bool {
        for block_type in matrix::BLOCK_TYPES {
            for block_no in 0..matrix::MATRIX_SIZE {
                let (row_range, col_range) = matrix::block_range(&block_type, block_no);

                let mut bmp: bitmap::Bitmap = 0;
                for row in row_range.into_iter() {
                    for col in col_range.into_iter() {
                        bmp |= self[(row, col)];

                        if bitmap::popcount(self[(row, col)]) > 1 {
                            return false;
                        }
                    }
                }

                if bmp != bitmap::FULL_BIT {
                    return false;
                }
            }
        }
        true
    }

    fn _prune_by_pivot(
        &self,
        pivot: matrix::Address,
        target_bit: bitmap::Bitmap,
    ) -> Option<matrix::Matrix> {
        let mut x = self.clone();

        for block_type in matrix::BLOCK_TYPES {
            let block_no = matrix::addr_to_block_no(&block_type, pivot);

            let (row_range, col_range) = matrix::block_range(&block_type, block_no);

            for row in row_range.into_iter() {
                for col in col_range.into_iter() {
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

        matrix::test_blocks_by_pivot(x, pivot)
    }

    pub fn solve(&self, cell_no: usize) -> Self {
        let mut y = self.clone();
        if cell_no >= matrix::MATRIX_SIZE * matrix::MATRIX_SIZE {
            return y;
        }

        let pivot = matrix::cell_no_to_addr(cell_no);
        let bits = bitmap::split_to_single_bits(self[pivot]);

        for target_bit in bits.into_iter() {
            y = match self._prune_by_pivot(pivot, target_bit) {
                Some(z) => z,
                None => {
                    continue;
                }
            };

            y = y.solve(cell_no + 1);

            if y._done() {
                return y;
            };
        }
        y
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
        let x = matrix::Matrix::new([
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
