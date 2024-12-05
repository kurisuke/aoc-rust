use std::cmp::Ordering;

use common::day::Day;

pub struct Day02 {}

impl Day for Day02 {
    fn star1(&self, input: &str) -> String {
        let safe_reports = input
            .lines()
            .map(Report::parse)
            .filter(|r| r.is_safe())
            .count();
        format!("{}", safe_reports)
    }

    fn star2(&self, input: &str) -> String {
        let safe_reports = input
            .lines()
            .map(Report::parse)
            .filter(|r| r.is_safe_tolerate())
            .count();
        format!("{}", safe_reports)
    }
}

struct Report(Vec<usize>);

impl Report {
    fn parse(line: &str) -> Self {
        Self(
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect(),
        )
    }

    fn is_safe(&self) -> bool {
        let mut order = Order::Unknown;

        for n in self.0.windows(2) {
            match order {
                Order::Unknown => match n[0].cmp(&n[1]) {
                    Ordering::Less => {
                        order = Order::Increasing;
                        if n[1] - n[0] > 3 {
                            return false;
                        }
                    }
                    Ordering::Greater => {
                        order = Order::Decreasing;
                        if n[0] - n[1] > 3 {
                            return false;
                        }
                    }
                    Ordering::Equal => {
                        return false;
                    }
                },
                Order::Increasing => {
                    if n[0] >= n[1] || n[1] - n[0] > 3 {
                        return false;
                    }
                }
                Order::Decreasing => {
                    if n[1] >= n[0] || n[0] - n[1] > 3 {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn is_safe_tolerate(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for i in 0..self.0.len() {
            if self.remove(i).is_safe() {
                return true;
            }
        }
        false
    }

    fn remove(&self, n: usize) -> Self {
        let mut report = self.0.clone();
        report.remove(n);
        Self(report)
    }
}

enum Order {
    Unknown,
    Increasing,
    Decreasing,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn star1() {
        let d = Day02 {};
        assert_eq!(d.star1(INPUT), "2");
    }

    #[test]
    fn star2() {
        let d = Day02 {};
        assert_eq!(d.star2(INPUT), "4");
    }
}
