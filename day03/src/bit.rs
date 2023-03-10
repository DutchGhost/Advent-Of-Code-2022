/// Within the range 97..=123 or 65..=91,
/// toggles bit 0..26 for the first range and 27..52 for the second.
pub trait ToBit {
    fn to_bit(self) -> u64;
}

impl ToBit for u8 {
    fn to_bit(self) -> u64 {
        1 << match self {
            97..=123 => self - 97,
            65..=91 => self - 65 + 26,
            _ => panic!(),
        }
    }
}

impl ToBit for &'_ u8 {
    fn to_bit(self) -> u64 {
        (*self).to_bit()
    }
}

impl ToBit for char {
    fn to_bit(self) -> u64 {
        u8::to_bit(self as u8)
    }
}
