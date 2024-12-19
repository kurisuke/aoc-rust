use std::collections::HashMap;

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

    fn star2(&self, input: &str) -> String {
        let (patterns, designs) = parse_input(input);
        let mut cache = HashMap::new();
        designs
            .iter()
            .map(|&design| variants(design, &patterns, &mut cache))
            .sum::<usize>()
            .to_string()
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

fn variants<'a>(design: &'a str, patterns: &[&str], cache: &mut HashMap<&'a str, usize>) -> usize {
    if design.is_empty() {
        1
    } else {
        patterns
            .iter()
            .filter(|&pattern| design.starts_with(pattern))
            .map(|&pattern| {
                let suffix = &design[pattern.len()..];
                if let Some(v) = cache.get(suffix) {
                    *v
                } else {
                    let v = variants(suffix, patterns, cache);
                    cache.insert(suffix, v);
                    v
                }
            })
            .sum()
    }
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
    fn star1() {
        let d = Day19 {};
        assert_eq!(d.star1(INPUT), "6");
    }

    #[test]
    fn star2() {
        let d = Day19 {};
        assert_eq!(d.star2(INPUT), "16");
    }
}
