pub mod matrix;

fn _done(x: &matrix::Matrix) -> bool {
    for block_type in matrix::BLOCK_TYPES {
        for block_no in 0..matrix::MATRIX_SIZE {
            let area = matrix::block_range(&block_type, block_no);

            let mut bmp = 0;
            for row_no in area.row_range.0..area.row_range.1 {
                for col_no in area.col_range.0..area.col_range.1 {
                    bmp |= x[row_no][col_no];

                    if matrix::bitmap::popcount(x[row_no][col_no]) > 1 {
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
    bit: matrix::bitmap::Bitmap,
) -> Option<matrix::Matrix> {
    let mut y = x.clone();

    for block_type in matrix::BLOCK_TYPES {
        let block_no = matrix::addr_to_block_no(&block_type, pivot);

        let area = matrix::block_range(&block_type, block_no);

        for row_no in area.row_range.0..area.row_range.1 {
            for col_no in area.col_range.0..area.col_range.1 {
                if row_no == pivot.row && col_no == pivot.col {
                    y[row_no][col_no] = bit;
                    continue;
                }

                y[row_no][col_no] &= !bit;

                if y[row_no][col_no] == 0 {
                    return None;
                }
            }
        }
    }
    Some(y)
}

pub fn bruteforce(x: &matrix::Matrix, cell_no: usize) -> matrix::Matrix {
    if cell_no >= matrix::MATRIX_SIZE * matrix::MATRIX_SIZE {
        return *x;
    }

    let addr = matrix::cell_no_to_addr(cell_no);
    let bits = matrix::bitmap::split_single_bit(x[addr.row][addr.col]);

    let mut y = [[0; matrix::MATRIX_SIZE]; matrix::MATRIX_SIZE];

    for bit in bits {
        y = match _prune_by_pivot(x, &addr, bit) {
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

    #[test]
    fn bruteforce_test() {
        let x = [
            [511, 511, 8, 511, 16, 511, 511, 511, 1],
            [511, 511, 32, 511, 511, 511, 511, 4, 511],
            [16, 4, 511, 64, 511, 511, 511, 511, 128],
            [1, 2, 511, 511, 32, 511, 511, 128, 511],
            [511, 511, 4, 511, 511, 511, 511, 511, 511],
            [511, 511, 511, 511, 511, 256, 511, 511, 64],
            [8, 511, 511, 511, 511, 511, 511, 511, 511],
            [128, 16, 511, 511, 1, 511, 511, 2, 511],
            [511, 511, 511, 32, 511, 511, 1, 511, 511],
        ];
        matrix::disp(&x);

        let y = bruteforce(&x, 0);

        matrix::disp(&y);
        assert!(_done(&y));
    }
}
