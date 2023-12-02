use aho_corasick::AhoCorasick;
use common::day::Day;

pub struct Day01 {}

impl Day for Day01 {
    fn star1(&self, input: &str) -> String {
        input
            .lines()
            .map(|l| calibration_value_1(l).unwrap())
            .sum::<u32>()
            .to_string()
    }

    fn star2(&self, input: &str) -> String {
        let digits = &[
            "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
            "six", "seven", "eight", "nine",
        ];
        let ac = AhoCorasick::new(digits).unwrap();

        input
            .lines()
            .map(|l| calibration_value_2(l, &ac).unwrap())
            .sum::<usize>()
            .to_string()
    }
}

fn calibration_value_1(line: &str) -> Option<u32> {
    let first = line
        .chars()
        .find(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap());
    let last = line
        .chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap());

    first.zip(last).map(|(first, last)| first * 10 + last)
}

fn calibration_value_2(line: &str, ac: &AhoCorasick) -> Option<usize> {
    let first = ac.find(line).map(|m| m.pattern().as_usize() % 9 + 1);
    let last = ac
        .find_overlapping_iter(line)
        .last()
        .map(|m| m.pattern().as_usize() % 9 + 1);

    first.zip(last).map(|(first, last)| first * 10 + last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        let d = Day01 {};
        assert_eq!(d.star1(input), "142");
    }

    #[test]
    fn star2() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        let d = Day01 {};
        assert_eq!(d.star2(input), "281");
    }
}
