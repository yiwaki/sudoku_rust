use std::fmt;
use std::ops;

pub mod bitmap;
use bitmap::BITMAP_DIGIT;

pub const MATRIX_SIZE: usize = 9;
pub const SQUARE_SIZE: usize = 3;

// Matrix of                     Block No. of each block type
// Cell No.                      Row          Column       Square
// +--------------------------+  +---------+  +---------+  +---------+
// | 0  1  2  3  4  5  6  7  8|  |000000000|  |012345678|  |000111222|
// | 9 10 11 12 13 14 15 16 17|  |111111111|  |012345678|  |000111222|
// |18 19 20 21 22 23 24 25 26|  |222222222|  |012345678|  |000111222|
// |27 28 29 30 31 32 33 34 35|  |333333333|  |012345678|  |333444555|
// |36 37 38 39 40 41 42 43 44|  |444444444|  |012345678|  |333444555|
// |45 46 47 48 49 50 51 52 53|  |555555555|  |012345678|  |333444555|
// |54 55 56 57 58 59 60 61 62|  |666666666|  |012345678|  |666777888|
// |63 64 65 66 67 68 69 70 71|  |777777777|  |012345678|  |666777888|
// |72 73 74 75 76 77 78 79 80|  |888888888|  |012345678|  |666777888|
// +--------------------------+  +---------+  +---------+  +---------+
//
// Address on Matrix: (Row No., Column No.)

#[derive(PartialEq)]
pub enum Block {
    Row,
    Column,
    Square,
}

pub const BLOCK_TYPES: [Block; 3] = [Block::Row, Block::Column, Block::Square];

#[derive(Clone, Copy)]
pub struct Range {
    start: usize,
    end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Range { start, end }
    }
}

impl Iterator for Range {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start >= self.end {
            None
        } else {
            let c = self.start;
            self.start += 1;
            Some(c)
        }
    }
}

pub type Address = (usize, usize);

type MatrixBuffer = [[bitmap::Bitmap; MATRIX_SIZE]; MATRIX_SIZE];

#[allow(dead_code)]
pub fn display_matrix_buffer(x: &MatrixBuffer) {
    for row in x.iter() {
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}

#[derive(Clone)]
pub struct Matrix {
    buffer: MatrixBuffer,
}

impl Matrix {
    pub fn new() -> Self {
        Matrix {
            buffer: [[0; MATRIX_SIZE]; MATRIX_SIZE],
        }
    }
}

// implement Into trait for Matrix automatically
impl From<MatrixBuffer> for Matrix {
    fn from(x: MatrixBuffer) -> Self {
        Matrix { buffer: x }
    }
}

impl ops::Index<Address> for Matrix {
    type Output = bitmap::Bitmap;
    fn index(&self, addr: Address) -> &Self::Output {
        &self.buffer[addr.0][addr.1]
    }
}

impl ops::IndexMut<Address> for Matrix {
    fn index_mut(&mut self, addr: Address) -> &mut Self::Output {
        &mut self.buffer[addr.0][addr.1]
    }
}

impl ops::Deref for Matrix {
    type Target = MatrixBuffer;
    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.buffer.iter() {
            for cell in row.iter() {
                write!(f, "{:0w$b} ", cell, w = BITMAP_DIGIT)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn cell_no_to_addr(cell_no: usize) -> Address {
    (cell_no / MATRIX_SIZE, cell_no % MATRIX_SIZE)
}

pub fn addr_to_block_no(block_type: &Block, addr: Address) -> usize {
    match block_type {
        Block::Row => addr.0,
        Block::Column => addr.1,
        Block::Square => addr.0 / SQUARE_SIZE * SQUARE_SIZE + addr.1 / SQUARE_SIZE,
    }
}

pub fn block_range(block_type: &Block, block_no: usize) -> (Range, Range) {
    match block_type {
        Block::Row => (
            Range::new(block_no, block_no + 1),
            Range::new(0, MATRIX_SIZE),
        ),
        Block::Column => (
            Range::new(0, MATRIX_SIZE),
            Range::new(block_no, block_no + 1),
        ),
        Block::Square => (
            Range::new(
                block_no / SQUARE_SIZE * SQUARE_SIZE,
                block_no / SQUARE_SIZE * SQUARE_SIZE + SQUARE_SIZE,
            ),
            Range::new(
                block_no % SQUARE_SIZE * SQUARE_SIZE,
                block_no % SQUARE_SIZE * SQUARE_SIZE + SQUARE_SIZE,
            ),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_no_to_addr_test() {
        assert_eq!(cell_no_to_addr(5), (0, 5));
    }

    #[test]
    fn addr_to_block_no_test() {
        let addr = (1, 2);
        let block_no = addr_to_block_no(&Block::Row, addr);
        assert_eq!(block_no, 1);

        let block_no = addr_to_block_no(&Block::Column, addr);
        assert_eq!(block_no, 2);

        let block_no = addr_to_block_no(&Block::Square, addr);
        assert_eq!(block_no, 0);
    }

    #[test]
    fn range_test() {
        let mut buf = String::new();
        let r = Range { start: 0, end: 3 };
        let c = Range { start: 0, end: 2 };
        for i in r {
            for j in c {
                print!("({},{}) ", i, j);
                buf.push_str(format!("({},{}) ", i, j).as_str());
            }
        }
        println!();
        assert_eq!(buf, "(0,0) (0,1) (1,0) (1,1) (2,0) (2,1) ");
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
