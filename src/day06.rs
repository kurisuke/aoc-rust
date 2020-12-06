use crate::day::Day;
use std::collections::HashSet;

pub struct Day06 {}

impl Day for Day06 {
    fn star1(&self, input: &str) -> String {
        let groups = input.split("\n\n");
        let sum_counts = groups
            .map(|g| questions_per_group(g))
            .fold(0, |sum_counts, q_set| sum_counts + q_set.len());
        format!("{}", sum_counts)
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

fn questions_per_group(group_str: &str) -> HashSet<char> {
    let questions: HashSet<_> = group_str.chars().filter(|c| !c.is_whitespace()).collect();
    questions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"
abc

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
    }
}
