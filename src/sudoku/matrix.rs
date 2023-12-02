use std::fmt;
use std::ops;

pub mod bitmap;

pub const MATRIX_SIZE: usize = 9;
pub const SQUARE_SIZE: usize = 3;

#[derive(Debug)]
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

#[derive(Clone)]
pub struct Matrix {
    buffer: MatrixBuffer,
}

impl Matrix {
    pub fn new(x: MatrixBuffer) -> Self {
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
        for row in 0..MATRIX_SIZE {
            for col in 0..MATRIX_SIZE {
                write!(f, "{:0w$b} ", self.buffer[row][col], w = MATRIX_SIZE)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
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

pub fn block_range(block_type: &Block, block_no: usize) -> (Range, Range) /* (row_range, col_range) */
{
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

pub fn test_blocks_by_pivot(x: Matrix, pivot: Address) -> Option<Matrix> {
    for block_type in BLOCK_TYPES {
        let block_no = addr_to_block_no(&block_type, pivot);
        let (row_range, col_range) = block_range(&block_type, block_no);

        let mut bmp: bitmap::Bitmap = 0;
        for row in row_range.into_iter() {
            for col in col_range.into_iter() {
                bmp |= x[(row, col)];
            }
        }

        if bmp != bitmap::FULL_BIT {
            if cfg!(debug_assertions) {
                println!("{:09b}:{:?}:{}-{:?}", bmp, block_type, block_no, pivot);
                println!("{}", x);
            }
            return None;
        }
    }
    Some(x)
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
        for i in r.into_iter() {
            for j in c.into_iter() {
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
