pub type Bitmap = u16;

pub const BITMAP_DIGIT: usize = 9;
pub const FULL_BIT: Bitmap = 0b111111111;

pub fn split_to_single_bits(bit: Bitmap) -> Vec<Bitmap> {
    let mut bits = Vec::new();

    for i in 0..BITMAP_DIGIT {
        let target_bit: Bitmap = 1 << i;
        if bit & target_bit != 0 {
            bits.push(target_bit);
        }
    }
    bits
}

pub fn popcount(bit: Bitmap) -> i32 {
    let mut cnt: i32 = 0;

    for i in 0..BITMAP_DIGIT {
        let target_bit: Bitmap = 1 << i;
        if bit & target_bit != 0 {
            cnt += 1;
        }
    }
    cnt
}

pub fn to_binary(bmp: Bitmap) -> String {
    let mut bin_str: String = String::new();

    for i in 0..BITMAP_DIGIT {
        if bmp & (0b100000000 >> i) == 0 {
            bin_str.push_str("0");
        } else {
            bin_str.push_str("1");
        }
    }
    bin_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_single_bit_test() {
        let bits = split_to_single_bits(0b000000000);
        assert_eq!(bits, []);

        let bits = split_to_single_bits(0b100100101);
        assert_eq!(bits, [1, 4, 32, 256]);

        let bits = split_to_single_bits(0b111111111);
        assert_eq!(bits, [1, 2, 4, 8, 16, 32, 64, 128, 256]);
    }

    #[test]
    fn popcount_test() {
        let cnt = popcount(0b000000000);
        assert_eq!(cnt, 0);

        let cnt = popcount(0b100100101);
        assert_eq!(cnt, 4);

        let cnt = popcount(0b111111111);
        assert_eq!(cnt, 9);
    }

    #[test]
    fn to_bin_test() {
        let bin = to_binary(0b000000000);
        assert_eq!(bin, "000000000");

        let bin = to_binary(0b100100101);
        assert_eq!(bin, "100100101");

        let bin = to_binary(0b111111111);
        assert_eq!(bin, "111111111");
    }
}
