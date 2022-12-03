use common::day::Day;

use std::collections::HashSet;

pub struct Day03 {}

fn char_score(c: &char) -> u32 {
    match c {
        'A'..='Z' => *c as u32 - 0x40 + 26,
        'a'..='z' => *c as u32 - 0x60,
        _ => panic!("Unknown character: {}", c),
    }
}

impl Day for Day03 {
    fn star1(&self, input: &str) -> String {
        let sum_priorities: u32 = input
            .lines()
            .map(|line| {
                // split line in half
                let (left, right) = line.split_at(line.len() / 2);
                let compartments = [left, right];

                // get common item in both compartments
                let items: Vec<HashSet<_>> = compartments
                    .into_iter()
                    .map(|part| part.chars().collect())
                    .collect();
                let common: HashSet<_> = items[0].intersection(&items[1]).collect();
                char_score(common.into_iter().next().unwrap())
            })
            .sum();
        format!("{}", sum_priorities)
    }

    fn star2(&self, input: &str) -> String {
        let lines: Vec<_> = input.lines().collect();
        let sum_priorities: u32 = lines
            .chunks(3)
            .map(|group| {
                // get common item among 3 lines
                let group_items: Vec<HashSet<_>> =
                    group.iter().map(|elf| elf.chars().collect()).collect();
                let mut iter = group_items.iter();
                let base = iter.next().unwrap().clone();
                let common = iter.fold(base, |acc, set| acc.intersection(set).copied().collect());
                char_score(&common.into_iter().next().unwrap())
            })
            .sum();
        format!("{}", sum_priorities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day03 {};

        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

        assert_eq!(d.star1(input), "157");
        assert_eq!(d.star2(input), "70");
    }
}
