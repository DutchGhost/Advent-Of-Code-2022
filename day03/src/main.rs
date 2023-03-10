const INPUT: &str = include_str!("../input.txt");

mod bit;
use bit::ToBit;

fn to_bitset(s: impl IntoIterator<Item = impl ToBit>) -> u64 {
    s.into_iter().fold(0u64, |acc, item| {
        let bit = item.to_bit();

        acc | bit
    })
}

fn solve(input: impl AsRef<str>) -> u32 {
    input
        .as_ref()
        .lines()
        .map(str::as_bytes)
        .map(|bytes| bytes.split_at(bytes.len() / 2))
        .map(|(c1, c2)| {
            let bitset_c1 = to_bitset(c1.iter());
            let bitset_c2 = to_bitset(c2.iter());

            let common = bitset_c1 & bitset_c2;
            common.trailing_zeros() + 1
        })
        .sum()
}

fn main() {
    let sum = solve(INPUT);
    println!("Part 1: {sum}");
}

#[cfg(test)]
mod test {
    use super::solve;

    const SAMPLE: &'static str = include_str!("../sample.txt");

    #[test]
    fn sample_guide() {
        assert_eq!(solve(SAMPLE), 157)
    }
}
