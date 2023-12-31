#[derive(Clone, Copy, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_test() {
        let mut buf = String::new();
        let rows = Range::new(0, 3).clone();
        let cols = Range::new(0, 2);
        println!("{:?}", rows);
        println!("{:?}", cols);
        for row in rows {
            for col in cols {
                print!("({},{}) ", row, col);
                buf.push_str(format!("({},{}) ", row, col).as_str());
            }
        }
        println!();
        assert_eq!(buf, "(0,0) (0,1) (1,0) (1,1) (2,0) (2,1) ");
    }
}
