use std::collections::HashMap;

use common::day::Day;

pub struct Day01 {}

impl Day for Day01 {
    fn star1(&self, input: &str) -> String {
        let (mut left, mut right) = parse_input(input);
        left.sort_unstable();
        right.sort_unstable();
        format!(
            "{}",
            left.into_iter()
                .zip(right)
                .map(|(left, right)| (left - right).abs())
                .sum::<isize>()
        )
    }

    fn star2(&self, input: &str) -> String {
        let (left, right) = parse_input(input);
        let mut occurrence = HashMap::new();
        for n in right {
            let e = occurrence.entry(n).or_insert(0);
            *e += 1;
        }
        let sum = left
            .into_iter()
            .map(|n| {
                let o = occurrence.get(&n).unwrap_or(&0);
                n * o
            })
            .sum::<isize>();
        format!("{}", sum)
    }
}

fn parse_input(input: &str) -> (Vec<isize>, Vec<isize>) {
    input
        .lines()
        .map(|line| {
            let mut it = line.split_whitespace();
            (
                it.next().unwrap().parse::<isize>().unwrap(),
                it.next().unwrap().parse::<isize>().unwrap(),
            )
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn star1() {
        let d = Day01 {};
        assert_eq!(d.star1(INPUT), "11");
    }

    #[test]
    fn star2() {
        let d = Day01 {};
        assert_eq!(d.star2(INPUT), "31");
    }
}
