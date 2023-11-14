use std::io::{stdout, Write};

pub mod bitmap;

pub const MATRIX_SIZE: usize = 9;
pub const SQUARE_SIZE: usize = 3;

pub type Matrix = [[bitmap::Bitmap; MATRIX_SIZE]; MATRIX_SIZE];

pub enum Block {
    Row,
    Column,
    Square,
}

pub const BLOCK_TYPES: [Block; 3] = [Block::Row, Block::Column, Block::Square];

pub struct Address {
    pub row: usize,
    pub col: usize,
}

pub struct Area {
    pub row_range: (usize, usize),
    pub col_range: (usize, usize),
}

pub fn cell_no_to_addr(cell_no: usize) -> Address {
    Address {
        row: cell_no / MATRIX_SIZE,
        col: cell_no % MATRIX_SIZE,
    }
}

pub fn addr_to_block_no(block_type: &Block, addr: &Address) -> usize {
    match block_type {
        Block::Row => addr.row,
        Block::Column => addr.col,
        Block::Square => addr.row / SQUARE_SIZE * SQUARE_SIZE + addr.col / SQUARE_SIZE,
    }
}

pub fn block_range(block_type: &Block, block_no: usize) -> Area {
    let mut area = Area {
        row_range: (0, 0),
        col_range: (0, 0),
    };

    match block_type {
        Block::Row => {
            area.row_range.0 = block_no;
            area.row_range.1 = area.row_range.0 + 1;
            area.col_range.0 = 0;
            area.col_range.1 = MATRIX_SIZE;
        }

        Block::Column => {
            area.row_range.0 = 0;
            area.row_range.1 = MATRIX_SIZE;
            area.col_range.0 = block_no;
            area.col_range.1 = area.col_range.0 + 1;
        }

        Block::Square => {
            area.row_range.0 = block_no / SQUARE_SIZE * SQUARE_SIZE;
            area.row_range.1 = area.row_range.0 + SQUARE_SIZE;
            area.col_range.0 = block_no % SQUARE_SIZE * SQUARE_SIZE;
            area.col_range.1 = area.col_range.0 + SQUARE_SIZE;
        }
    }
    area
}

#[allow(dead_code)]
pub fn disp(x: &Matrix) {
    println!("");
    for row in 0..MATRIX_SIZE {
        for col in 0..MATRIX_SIZE {
            print!("{} ", bitmap::to_binary(x[row][col]));
        }
        println!("");
    }
    stdout().flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_no_to_addr_test() {
        let addr = cell_no_to_addr(5);
        assert_eq!(addr.row, 0);
        assert_eq!(addr.col, 5);
    }

    #[test]
    fn addr_to_block_no_test() {
        let addr = Address { row: 1, col: 2 };
        let block_no = addr_to_block_no(&Block::Row, &addr);
        assert_eq!(block_no, 1);

        let block_no = addr_to_block_no(&Block::Column, &addr);
        assert_eq!(block_no, 2);

        let block_no = addr_to_block_no(&Block::Square, &addr);
        assert_eq!(block_no, 0);
    }

    #[test]
    fn block_range_test() {
        let area = block_range(&Block::Row, 4);
        assert_eq!(area.row_range.0, 4);
        assert_eq!(area.row_range.1, 5);
        assert_eq!(area.col_range.0, 0);
        assert_eq!(area.col_range.1, MATRIX_SIZE);

        let area = block_range(&Block::Column, 4);
        assert_eq!(area.row_range.0, 0);
        assert_eq!(area.row_range.1, MATRIX_SIZE);
        assert_eq!(area.col_range.0, 4);
        assert_eq!(area.col_range.1, 5);

        let area = block_range(&Block::Square, 4);
        assert_eq!(area.row_range.0, 3);
        assert_eq!(area.row_range.1, 6);
        assert_eq!(area.col_range.0, 3);
        assert_eq!(area.col_range.1, 6);
    }
}
