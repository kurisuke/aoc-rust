use common::day::Day;

pub struct Day01 {}

fn parse_input(input: &str) -> impl Iterator<Item = usize> + '_ {
    input
        .split("\n\n")
        .map(|elf| elf.lines().fold(0, |a, l| a + l.parse::<usize>().unwrap()))
}

impl Day for Day01 {
    fn star1(&self, input: &str) -> String {
        let elves = parse_input(input);
        let max_calories = elves.max().unwrap();

        format!("{}", max_calories)
    }

    fn star2(&self, input: &str) -> String {
        let mut elves: Vec<_> = parse_input(input).collect();
        elves.sort_by(|a, b| b.cmp(a)); // descending sort
        let max3_calories: usize = elves.into_iter().take(3).sum();

        format!("{}", max3_calories)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

        let d = Day01 {};
        assert_eq!(d.star1(input), "24000");
        assert_eq!(d.star2(input), "45000");
    }
}
