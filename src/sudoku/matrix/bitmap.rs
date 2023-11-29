pub type Bitmap = u16;

pub const BITMAP_DIGIT: usize = 9;
pub const FULL_BIT: Bitmap = 0b1_1111_1111;

pub fn split_to_single_bits(bit: Bitmap) -> Vec<Bitmap> {
    let mut bits: Vec<Bitmap> = Vec::new();
    let mut target_bit: Bitmap = 1;

    for _ in 0..BITMAP_DIGIT {
        if bit & target_bit != 0 {
            bits.push(target_bit);
        }
        target_bit <<= 1;
    }
    bits
}

pub fn popcount(bit: Bitmap) -> usize {
    let mut cnt = 0;
    let mut target_bit: Bitmap = 1;

    for _ in 0..BITMAP_DIGIT {
        if bit & target_bit != 0 {
            cnt += 1;
        }
        target_bit <<= 1;
    }
    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_single_bit_test() {
        let bits = split_to_single_bits(0b0_0000_0000);
        assert_eq!(bits, []);

        let bits = split_to_single_bits(0b1_0010_0101);
        assert_eq!(bits, [1, 4, 32, 256]);

        let bits = split_to_single_bits(0b1_1111_1111);
        assert_eq!(bits, [1, 2, 4, 8, 16, 32, 64, 128, 256]);
    }

    #[test]
    fn popcount_test() {
        let cnt = popcount(0b0_0000_0000);
        assert_eq!(cnt, 0);

        let cnt = popcount(0b1_0010_0101);
        assert_eq!(cnt, 4);

        let cnt = popcount(0b1_1111_1111);
        assert_eq!(cnt, 9);
    }
}
