use common::day::Day;

pub struct Day19 {}

impl Day for Day19 {
    fn star1(&self, input: &str) -> String {
        let (patterns, designs) = parse_input(input);
        designs
            .iter()
            .filter(|&design| is_possible(design, &patterns))
            .count()
            .to_string()
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut secs = input.split("\n\n");

    let patterns = secs.next().unwrap().split(", ").collect();
    let designs = secs.next().unwrap().lines().collect();

    (patterns, designs)
}

fn is_possible(design: &str, patterns: &[&str]) -> bool {
    design.is_empty()
        || patterns.iter().any(|&pattern| {
            design.starts_with(pattern) && is_possible(&design[pattern.len()..], patterns)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"#;

    #[test]
    fn ex1() {
        let d = Day19 {};
        assert_eq!(d.star1(INPUT), "6");
    }
}
