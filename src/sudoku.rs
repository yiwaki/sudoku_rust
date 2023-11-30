pub mod matrix;

fn _done(x: &matrix::Matrix) -> bool {
    for block_type in matrix::BLOCK_TYPES {
        for block_no in 0..matrix::MATRIX_SIZE {
            let (row_range, col_range) = matrix::block_range(&block_type, block_no);

            let mut bmp: matrix::bitmap::Bitmap = 0;
            for row in row_range.into_iter() {
                for col in col_range.into_iter() {
                    bmp |= x[&(row, col)];

                    if matrix::bitmap::popcount(x[&(row, col)]) > 1 {
                        return false;
                    }
                }
            }

            if bmp != matrix::bitmap::FULL_BIT {
                return false;
            }
        }
    }
    true
}

fn _prune_by_pivot(
    x: &matrix::Matrix,
    pivot: &matrix::Address,
    target_bit: matrix::bitmap::Bitmap,
) -> Option<matrix::Matrix> {
    let mut y = x.clone();

    for block_type in matrix::BLOCK_TYPES {
        let block_no = matrix::addr_to_block_no(&block_type, pivot);

        let (row_range, col_range) = matrix::block_range(&block_type, block_no);

        for row in row_range.into_iter() {
            for col in col_range.into_iter() {
                if (row, col) == *pivot {
                    y[&(row, col)] = target_bit;
                    continue;
                }

                y[&(row, col)] &= !target_bit;

                if y[&(row, col)] == 0 {
                    return None;
                }
            }
        }
    }

    if matrix::test_bitmap_by_addr(&y, pivot) {
        Some(y)
    } else {
        None
    }
}

pub fn bruteforce(x: &matrix::Matrix, cell_no: usize) -> matrix::Matrix {
    let mut y = x.clone();
    if cell_no >= matrix::CELL_COUNT {
        return y;
    }

    let pivot = matrix::cell_no_to_addr(cell_no);
    let bits = matrix::bitmap::split_to_single_bits(x[&pivot]);

    for target_bit in bits.into_iter() {
        y = match _prune_by_pivot(x, &pivot, target_bit) {
            Some(z) => z,
            None => {
                continue;
            }
        };

        y = bruteforce(&y, cell_no + 1);

        if _done(&y) {
            return y;
        };
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;

    fn _check_problem(problem: &matrix::Matrix, solution: &matrix::Matrix) -> bool {
        for row in 0..matrix::MATRIX_SIZE {
            for col in 0..matrix::MATRIX_SIZE {
                if problem[&(row, col)] != matrix::bitmap::FULL_BIT
                    && problem[&(row, col)] != solution[&(row, col)]
                {
                    return false;
                }
            }
        }
        _done(solution)
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

        let y = bruteforce(&x, 0);

        println!("Solution:");
        println!("{}", y);
        assert!(_check_problem(&x, &y));
    }
}
