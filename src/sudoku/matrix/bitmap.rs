use std::iter;

pub type Bitmap = u16;

pub const BITMAP_DIGIT: usize = 9;
pub const FULL_BIT: Bitmap = (1 << BITMAP_DIGIT) - 1;

pub struct ForEachBit {
    bitmap: Bitmap,
    next_bit: Bitmap,
}

impl ForEachBit {
    pub fn new(bitmap: Bitmap) -> Self {
        ForEachBit {
            bitmap,
            next_bit: 1 << (BITMAP_DIGIT - 1),
        }
    }
}

impl Iterator for ForEachBit {
    type Item = Bitmap;

    fn next(&mut self) -> Option<Self::Item> {
        iter::successors(Some(self.next_bit), |&bit| (bit > 1).then_some(bit >> 1))
            .skip_while(|&bit| bit == 0)
            .find(|&bit| self.bitmap & bit != 0)
            .or_else(|| {
                self.next_bit = 0;
                None
            })
            .inspect(|&bit| self.next_bit = bit >> 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_single_bit_test() {
        let v = ForEachBit::new(0b000_000_000).next();
        assert_eq!(v, None);

        let mut bits = Vec::<Bitmap>::new();
        for v in ForEachBit::new(0b100_100_101) {
            bits.push(v);
        }
        assert_eq!(bits, [256, 32, 4, 1]);

        let mut bits = Vec::<Bitmap>::new();
        for v in ForEachBit::new(FULL_BIT) {
            bits.push(v);
        }
        assert_eq!(bits, [256, 128, 64, 32, 16, 8, 4, 2, 1]);
    }
}
