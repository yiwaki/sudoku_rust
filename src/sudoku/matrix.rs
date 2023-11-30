use std::fmt;
use std::ops::{Deref, Index, IndexMut};

pub mod bitmap;

pub const MATRIX_SIZE: usize = 9;
pub const SQUARE_SIZE: usize = 3;
pub const CELL_COUNT: usize = MATRIX_SIZE * MATRIX_SIZE;

#[derive(Debug)]
pub enum Block {
    Row,
    Column,
    Square,
}

pub const BLOCK_TYPES: [Block; 3] = [Block::Row, Block::Column, Block::Square];

#[derive(Debug)]
pub struct Address {
    pub row: usize,
    pub col: usize,
}

impl PartialEq for Address {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Iterator for Range {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let c = self.start;
            self.start += 1;
            Some(c)
        }
    }
}

type MatrixBuffer = [[bitmap::Bitmap; MATRIX_SIZE]; MATRIX_SIZE];

#[derive(Clone)]
pub struct Matrix {
    buffer: MatrixBuffer,
}

impl Matrix {
    pub fn new(x: MatrixBuffer) -> Matrix {
        Matrix { buffer: x }
    }
}

impl Index<&Address> for Matrix {
    type Output = bitmap::Bitmap;
    fn index(&self, index: &Address) -> &Self::Output {
        &self.buffer[index.row][index.col]
    }
}

impl IndexMut<&Address> for Matrix {
    fn index_mut(&mut self, index: &Address) -> &mut Self::Output {
        &mut self.buffer[index.row][index.col]
    }
}

impl Deref for Matrix {
    type Target = MatrixBuffer;
    fn deref(&self) -> &MatrixBuffer {
        &self.buffer
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..MATRIX_SIZE {
            for col in 0..MATRIX_SIZE {
                let _ = write!(f, "{:0w$b} ", self.buffer[row][col], w = MATRIX_SIZE);
            }
            let _ = writeln!(f);
        }
        writeln!(f)
    }
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

pub fn block_range(block_type: &Block, block_no: usize) -> (Range, Range) {
    match block_type {
        Block::Row => (
            Range {
                start: block_no,
                end: block_no + 1,
            },
            Range {
                start: 0,
                end: MATRIX_SIZE,
            },
        ),

        Block::Column => (
            Range {
                start: 0,
                end: MATRIX_SIZE,
            },
            Range {
                start: block_no,
                end: block_no + 1,
            },
        ),

        Block::Square => (
            Range {
                start: block_no / SQUARE_SIZE * SQUARE_SIZE,
                end: block_no / SQUARE_SIZE * SQUARE_SIZE + SQUARE_SIZE,
            },
            Range {
                start: block_no % SQUARE_SIZE * SQUARE_SIZE,
                end: block_no % SQUARE_SIZE * SQUARE_SIZE + SQUARE_SIZE,
            },
        ),
    }
}

pub fn test_bitmap_by_addr(x: &Matrix, addr: &Address) -> bool {
    for block_type in BLOCK_TYPES {
        let block_no = addr_to_block_no(&block_type, addr);
        let (row_range, col_range) = block_range(&block_type, block_no);

        let mut bmp: bitmap::Bitmap = 0;
        for row in (row_range.start)..(row_range.end) {
            for col in (col_range.start)..(col_range.end) {
                let addr = Address { row, col };
                bmp |= x[&addr];
            }
        }

        if bmp != bitmap::FULL_BIT {
            if cfg!(debug_assertions) {
                println!("{:09b}:{:?}:{}-{:?}", bmp, block_type, block_no, addr);
                println!("{}", x);
            }
            return false;
        }
    }
    true
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
        let (row_range, col_range) = block_range(&Block::Row, 4);
        assert_eq!(row_range.start, 4);
        assert_eq!(row_range.end, 5);
        assert_eq!(col_range.start, 0);
        assert_eq!(col_range.end, MATRIX_SIZE);

        let (row_range, col_range) = block_range(&Block::Column, 4);
        assert_eq!(row_range.start, 0);
        assert_eq!(row_range.end, MATRIX_SIZE);
        assert_eq!(col_range.start, 4);
        assert_eq!(col_range.end, 5);

        let (row_range, col_range) = block_range(&Block::Square, 4);
        assert_eq!(row_range.start, 3);
        assert_eq!(row_range.end, 6);
        assert_eq!(col_range.start, 3);
        assert_eq!(col_range.end, 6);
    }
}
