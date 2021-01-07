use common::day::Day;
use std::collections::HashSet;

pub struct Day01 {}

impl Day for Day01 {
    fn star1(&self, input: &str) -> String {
        let sum = input
            .lines()
            .map(|line| line.parse::<i64>().unwrap())
            .sum::<i64>();
        format!("{}", sum)
    }

    fn star2(&self, input: &str) -> String {
        let mut freq = 0;
        let mut seen = HashSet::new();
        seen.insert(0);

        let vals: Vec<_> = input.lines().map(|x| x.parse::<i64>().unwrap()).collect();
        let vlen = vals.len();
        let mut i = 0;
        loop {
            freq += vals[i % vlen];
            if !seen.insert(freq) {
                return format!("{}", freq);
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let d = Day01 {};
        assert_eq!(d.star1("+1\n-2\n+3\n+1"), "3");
        assert_eq!(d.star1("+1\n+1\n+1"), "3");
        assert_eq!(d.star1("+1\n+1\n-2"), "0");
        assert_eq!(d.star1("-1\n-2\n-3"), "-6");
    }

    #[test]
    fn star2() {
        let d = Day01 {};
        assert_eq!(d.star2("+1\n-2\n+3\n+1"), "2");
        assert_eq!(d.star2("+1\n-1"), "0");
        assert_eq!(d.star2("+3\n+3\n+4\n-2\n-4"), "10");
        assert_eq!(d.star2("-6\n+3\n+8\n+5\n-6"), "5");
        assert_eq!(d.star2("+7\n+7\n-2\n-7\n-4"), "14");
    }
}
