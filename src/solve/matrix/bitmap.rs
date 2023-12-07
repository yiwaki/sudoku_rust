pub type Bitmap = u16;

pub const BITMAP_DIGIT: usize = 9;
pub const FULL_BIT: Bitmap = 0b1_1111_1111;

pub struct ForEachBit {
    bit_buffer: Bitmap,
    current_bit: Bitmap,
}

impl ForEachBit {
    pub fn from(bitmap: Bitmap) -> Self {
        ForEachBit {
            bit_buffer: bitmap,
            current_bit: 0b1_0000_0000,
        }
    }
}

impl Iterator for ForEachBit {
    type Item = Bitmap;
    fn next(&mut self) -> Option<Self::Item> {
        while self.bit_buffer & self.current_bit == 0 && self.current_bit != 0 {
            self.current_bit >>= 1;
        }

        if self.current_bit == 0 {
            None
        } else {
            let cur = self.current_bit;
            self.current_bit >>= 1;
            Some(cur)
        }
    }
}

pub fn popcount(bitmap: Bitmap) -> usize {
    let mut count = 0;
    let mut target_bit: Bitmap = 1;

    for _ in 0..BITMAP_DIGIT {
        if bitmap & target_bit != 0 {
            count += 1;
        }
        target_bit <<= 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_single_bit_test() {
        let mut bits = Vec::<Bitmap>::new();
        for v in ForEachBit::from(0b0_0000_0000) {
            bits.push(v);
        }
        assert_eq!(bits, []);

        let mut bits = Vec::<Bitmap>::new();
        for v in ForEachBit::from(0b1_0010_0101) {
            bits.push(v);
        }
        assert_eq!(bits, [256, 32, 4, 1]);

        let mut bits = Vec::<Bitmap>::new();
        for v in ForEachBit::from(0b1_1111_1111) {
            bits.push(v);
        }
        assert_eq!(bits, [256, 128, 64, 32, 16, 8, 4, 2, 1]);
    }

    #[test]
    fn popcount_test() {
        assert_eq!(popcount(0b0_0000_0000), 0);
        assert_eq!(popcount(0b1_0010_0101), 4);
        assert_eq!(popcount(0b1_1111_1111), 9);
    }
}
