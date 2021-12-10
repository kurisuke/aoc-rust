use common::day::Day;
use std::collections::{HashMap, HashSet};

pub struct Day06 {}

impl Day for Day06 {
    fn star1(&self, input: &str) -> String {
        let groups = input.split("\n\n");
        let sum_counts: usize = groups.map(any_questions_per_group).sum();
        format!("{}", sum_counts)
    }

    fn star2(&self, input: &str) -> String {
        let groups = input.split("\n\n");
        let sum_counts: usize = groups.map(all_questions_per_group).sum();
        format!("{}", sum_counts)
    }
}

fn any_questions_per_group(group_str: &str) -> usize {
    let questions: HashSet<_> = group_str.chars().filter(|c| !c.is_whitespace()).collect();
    questions.len()
}

fn all_questions_per_group(group_str: &str) -> usize {
    let lines: Vec<_> = group_str.lines().collect();
    let mut answers = HashMap::new();
    for line in &lines {
        for ch in line.chars() {
            let counter = answers.entry(ch).or_insert(0);
            *counter += 1;
        }
    }
    answers.values().filter(|&v| *v == lines.len()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"abc

a
b
c

ab
ac

a
a
a
a

b
"#;
        let d = Day06 {};
        assert_eq!(d.star1(input), "11");
        assert_eq!(d.star2(input), "6");
    }
}
