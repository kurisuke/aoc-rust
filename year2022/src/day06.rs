use common::day::Day;

use std::collections::HashSet;

pub struct Day06 {}

fn find_start_marker(input: &str, n: usize) -> usize {
    let chars: Vec<_> = input.chars().collect();
    chars
        .windows(n)
        .enumerate()
        .find(|(_, c)| {
            let set: HashSet<_> = c.iter().collect();
            n == set.len()
        })
        .map(|(i, _)| i + n)
        .unwrap()
}

impl Day for Day06 {
    fn star1(&self, input: &str) -> String {
        format!("{}", find_start_marker(input, 4))
    }

    fn star2(&self, input: &str) -> String {
        format!("{}", find_start_marker(input, 14))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day06 {};
        assert_eq!(d.star1("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5");
        assert_eq!(d.star1("nppdvjthqldpwncqszvftbrmjlhg"), "6");
        assert_eq!(d.star1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "10");
        assert_eq!(d.star1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "11");
    }

    #[test]
    fn ex2() {
        let d = Day06 {};
        assert_eq!(d.star2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "19");
        assert_eq!(d.star2("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23");
        assert_eq!(d.star2("nppdvjthqldpwncqszvftbrmjlhg"), "23");
        assert_eq!(d.star2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "29");
        assert_eq!(d.star2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "26");
    }
}
