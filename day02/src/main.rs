use core::{convert::AsRef, iter::Sum, ops::Add, str::FromStr};

mod shape;
mod strategy;

pub(crate) use shape::Shape;
pub(crate) use strategy::{Choice, Guide, Strategy};

const INPUT: &'static str = include_str!("../input.txt");

fn solve<S: Strategy>(input: impl AsRef<str>) -> u32 {
    input
        .as_ref()
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let them = Shape::from(bytes[0]);
            let you = Shape::from(bytes[2]);

            (them, you)
        })
        .map(|(them, you)| S::strategy(you, them))
        .sum()
}

fn main() {
    let score_guide = solve::<Guide>(INPUT);
    let score_choice = solve::<Choice>(INPUT);
    println!("Part 1: {score_guide}\nPart2: {score_choice}");
}

#[cfg(test)]
mod test {
    use super::{solve, Choice, Guide, Shape};

    const SAMPLE: &'static str = include_str!("../sample.txt");

    #[test]
    fn sample_guide() {
        assert_eq!(solve::<Guide>(SAMPLE), 15);
    }
    #[test]
    fn sample_choice() {
        assert_eq!(solve::<Choice>(SAMPLE), 12);
    }
}
