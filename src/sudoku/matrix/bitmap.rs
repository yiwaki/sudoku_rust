pub type Bitmap = u16;

pub const BITMAP_DIGIT: usize = 9;
pub const FULL_BIT: Bitmap = 0b1_1111_1111;

pub fn each_bit(bitmap: Bitmap) -> Vec<Bitmap> {
    let mut each_bit: Vec<Bitmap> = Vec::new();
    let mut target_bit: Bitmap = 1;

    for _ in 0..BITMAP_DIGIT {
        if bitmap & target_bit != 0 {
            each_bit.push(target_bit);
        }
        target_bit <<= 1;
    }
    each_bit
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
        let bits = each_bit(0b0_0000_0000);
        assert_eq!(bits, []);

        let bits = each_bit(0b1_0010_0101);
        assert_eq!(bits, [1, 4, 32, 256]);

        let bits = each_bit(0b1_1111_1111);
        assert_eq!(bits, [1, 2, 4, 8, 16, 32, 64, 128, 256]);
    }

    #[test]
    fn popcount_test() {
        assert_eq!(popcount(0b0_0000_0000), 0);
        assert_eq!(popcount(0b1_0010_0101), 4);
        assert_eq!(popcount(0b1_1111_1111), 9);
    }
}
