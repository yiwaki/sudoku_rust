pub mod matrix;
use matrix::bitmap;

impl matrix::Matrix {
    fn _done(self) -> Option<Self> {
        for block_no in 0..matrix::MATRIX_SIZE {
            for block_type in matrix::BLOCK_TYPES {
                let mut bitmap = 0;

                let (row_range, col_range) = matrix::block_range(&block_type, block_no);

                for row in row_range.into_iter() {
                    for col in col_range.into_iter() {
                        bitmap |= self[(row, col)];

                        if block_type == matrix::Block::Row
                            && bitmap::popcount(self[(row, col)]) > 1
                        {
                            return None;
                        }
                    }
                }

                if bitmap != matrix::bitmap::FULL_BIT {
                    return None;
                }
            }
        }
        Some(self)
    }

    fn _check_blocks_by_pivot(self, pivot: matrix::Address) -> Option<Self> {
        for block_type in matrix::BLOCK_TYPES {
            let block_no = matrix::addr_to_block_no(&block_type, pivot);

            let (row_range, col_range) = matrix::block_range(&block_type, block_no);

            let mut bitmap: bitmap::Bitmap = 0;
            for row in row_range.into_iter() {
                for col in col_range.into_iter() {
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
        x._check_blocks_by_pivot(pivot)
    }

    pub fn solve(&self, cell_no: usize) -> Self {
        if cell_no >= matrix::MATRIX_SIZE * matrix::MATRIX_SIZE {
            return self.clone();
        }

        let mut x = matrix::Matrix::new();

        let pivot = matrix::cell_no_to_addr(cell_no);

        for target_bit in bitmap::ForEachBit::from(self[pivot]) {
            x = match self._pruned_by_pivot(pivot, target_bit) {
                Some(y) => y,
                None => continue,
            };

            x = match x.solve(cell_no + 1)._done() {
                Some(y) => return y,
                None => x,
            };
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _check_problem_solution(problem: matrix::Matrix, solution: matrix::Matrix) -> bool {
        for row in 0..matrix::MATRIX_SIZE {
            for col in 0..matrix::MATRIX_SIZE {
                if problem[(row, col)] != matrix::bitmap::FULL_BIT
                    && problem[(row, col)] != solution[(row, col)]
                {
                    return false;
                }
            }
        }
        solution._done().is_some()
    }

    #[test]
    fn solve_test() {
        let x: matrix::Matrix = [
            [1, 3, 0, 7, 9, 0, 0, 4, 5],
            [0, 0, 5, 0, 0, 0, 0, 0, 6],
            [2, 0, 0, 0, 8, 0, 9, 0, 0],
            [3, 1, 0, 4, 6, 5, 8, 0, 0],
            [4, 0, 0, 0, 0, 0, 0, 0, 2],
            [0, 0, 7, 9, 3, 2, 0, 1, 4],
            [0, 0, 3, 0, 7, 0, 0, 0, 8],
            [7, 0, 0, 0, 0, 0, 3, 0, 0],
            [8, 9, 0, 0, 5, 4, 0, 2, 1],
        ]
        .map(|y| {
            y.map(|z| {
                if z == 0 {
                    bitmap::FULL_BIT
                } else {
                    1 << (z - 1)
                }
            })
        })
        .into();

        println!("Problem:");
        println!("{}", x);

        let y = x.solve(0);

        println!("Solution:");
        println!("{}", y);

        assert!(_check_problem_solution(x, y));
    }
}
