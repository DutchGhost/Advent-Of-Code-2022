use core::{convert::AsRef, iter::Sum, ops::Add, str::FromStr};

const INPUT: &'static str = include_str!("../input.txt");

fn solve<T>(input: impl AsRef<str>) -> (T, T)
where
    T: Copy + FromStr + Sum + Ord + Default + Add<Output = T>,
{
    let [min, mid, max] = input
        .as_ref()
        .split("\r\n\r\n")
        .map(|calories| {
            calories
                .lines()
                .filter_map(|line| line.parse::<T>().ok())
                .sum()
        })
        .fold([T::default(), T::default(), T::default()], |max, item| {
            let mut a = [max[0], max[1], max[2], item];
            a.sort();
            [a[1], a[2], a[3]]
        });

    (max, min + mid + max)
}

fn main() {
    let (p1, p2) = solve::<u32>(INPUT);
    print!("Part 1: {p1}\nPart 2:  {p2}");
}

#[cfg(test)]
mod test {
    use super::solve;

    const SAMPLE: &'static str = include_str!("../sample.txt");

    #[test]
    fn sample() {
        assert_eq!(solve::<u32>(SAMPLE), (24000, 45000));
    }
}
