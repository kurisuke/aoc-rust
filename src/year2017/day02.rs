use crate::day::Day;
use itertools::Itertools;

pub struct Day02 {}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

impl Day for Day02 {
    fn star1(&self, input: &str) -> String {
        let rows = parse_input(input);
        let checksum = rows
            .iter()
            .map(|row| {
                let max = row.iter().max().unwrap();
                let min = row.iter().min().unwrap();
                max - min
            })
            .sum::<usize>();
        format!("{}", checksum)
    }

    fn star2(&self, input: &str) -> String {
        let rows = parse_input(input);
        let checksum = rows
            .iter()
            .map(|row| {
                let div_ops = row
                    .iter()
                    .permutations(2)
                    .find(|n| n[0] % n[1] == 0)
                    .unwrap();
                div_ops[0] / div_ops[1]
            })
            .sum::<usize>();
        format!("{}", checksum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let d = Day02 {};
        let input = r#"5 1 9 5
7 5 3
2 4 6 8"#;
        assert_eq!(d.star1(input), "18");
    }

    #[test]
    fn star2() {
        let d = Day02 {};
        let input = r#"5 9 2 8
9 4 7 3
3 8 6 5"#;
        assert_eq!(d.star2(input), "9");
    }
}
