const INPUT: &'static str = include_str!("../input.txt");

use std::collections::HashSet;

fn solve(input: impl AsRef<str>) -> u32 {
    input
        .as_ref()
        .lines()
        .map(str::as_bytes)
        .map(|bytes| bytes.split_at(bytes.len() / 2))
        .map(|(r1, r2)| {
            // really.... why are we using hashsets?
            (
                r1.iter().copied().collect::<HashSet<_>>(),
                r2.iter().copied().collect::<HashSet<_>>(),
            )
        })
        .filter_map(|(r1, r2)| {
            r1.intersection(&r2)
                .map(|letter| match letter {
                    b'a'..=b'z' => letter - 96,
                    b'A'..=b'Z' => letter - 64 + 26,
                    _ => panic!("not a good range"),
                })
                .map(|common| common as u32)
                .nth(0)
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
