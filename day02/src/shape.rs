use core::convert::From;

#[derive(Copy, Clone)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<u8> for Shape {
    fn from(byte: u8) -> Self {
        match byte {
            b'A' | b'X' => Self::Rock,
            b'B' | b'Y' => Self::Paper,
            b'C' | b'Z' => Self::Scissors,
            _ => panic!("Input expected to be A | X, B | Y, C | Z"),
        }
    }
}
