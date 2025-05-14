pub const LAST_BIT_MASK: u16 = 0b1000_0000_0000_0000;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct GrayCode(pub u16);

impl From<BinaryCode> for GrayCode {
    fn from(code: BinaryCode) -> Self {
        let mut result = code.0 & LAST_BIT_MASK;

        for i in 1..16 {
            let previous_bit = (code.0 & (LAST_BIT_MASK >> (i - 1))) >> 1;
            let current_bit = code.0 & (LAST_BIT_MASK >> i);
            result |= previous_bit ^ current_bit;
        }

        GrayCode(result)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct BinaryCode(pub u16);

impl From<GrayCode> for BinaryCode {
    fn from(code: GrayCode) -> Self {
        let mut value = code.0 & LAST_BIT_MASK;
        let mut result = value;

        for i in 1..16 {
            if (code.0 & (LAST_BIT_MASK >> i)) << i == LAST_BIT_MASK {
                value = !value;
            }

            result |= (value & LAST_BIT_MASK) >> i;
        }

        BinaryCode(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_bin_to_gray() {
        assert_eq!(
            GrayCode::from(BinaryCode(0b0000_0000_0000_0000)),
            GrayCode(0b0000_0000_0000_0000)
        );
        assert_eq!(
            GrayCode::from(BinaryCode(0b0000_0000_0000_0001)),
            GrayCode(0b0000_0000_0000_0001)
        );
        assert_eq!(
            GrayCode::from(BinaryCode(0b0000_0000_0000_0010)),
            GrayCode(0b0000_0000_0000_0011)
        );
        assert_eq!(
            GrayCode::from(BinaryCode(0b0000_0000_0000_0011)),
            GrayCode(0b0000_0000_0000_0010)
        );
    }

    #[test]
    fn converts_gray_to_bin() {
        assert_eq!(
            BinaryCode::from(GrayCode(0b0000_0000_0000_0000)),
            BinaryCode(0b0000_0000_0000_0000)
        );
        assert_eq!(
            BinaryCode::from(GrayCode(0b0000_0000_0000_0001)),
            BinaryCode(0b0000_0000_0000_0001)
        );
        assert_eq!(
            BinaryCode::from(GrayCode(0b0000_0000_0000_0011)),
            BinaryCode(0b0000_0000_0000_0010)
        );
        assert_eq!(
            BinaryCode::from(GrayCode(0b0000_0000_0000_0010)),
            BinaryCode(0b0000_0000_0000_0011)
        );
    }
}
