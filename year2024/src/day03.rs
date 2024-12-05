use common::day::Day;
use regex::Regex;

pub struct Day03 {}

impl Day for Day03 {
    fn star1(&self, input: &str) -> String {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let sum = re
            .captures_iter(input)
            .map(|c| c.extract())
            .map(|(_, [n1, n2])| n1.parse::<usize>().unwrap() * n2.parse::<usize>().unwrap())
            .sum::<usize>();
        format!("{}", sum)
    }

    fn star2(&self, input: &str) -> String {
        let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();
        let mut mul_enabled = true;
        let mut sum = 0;

        for c in re.captures_iter(input) {
            if c.get(3).is_some() {
                // do
                mul_enabled = true;
            } else if c.get(4).is_some() {
                // don't
                mul_enabled = false;
            } else if let (Some(n1), Some(n2)) = (c.get(1), c.get(2)) {
                if mul_enabled {
                    sum += n1.as_str().parse::<usize>().unwrap()
                        * n2.as_str().parse::<usize>().unwrap()
                }
            } else {
                unreachable!()
            }
        }

        format!("{}", sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let input = r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

        let d = Day03 {};
        assert_eq!(d.star1(input), "161");
    }

    #[test]
    fn star2() {
        let input = r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

        let d = Day03 {};
        assert_eq!(d.star2(input), "48");
    }
}
