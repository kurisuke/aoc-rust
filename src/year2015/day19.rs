use crate::day::Day;
use std::collections::{HashMap, HashSet};

pub struct Day19 {}

type Replacements<'a> = HashMap<&'a str, Vec<&'a str>>;

struct InputInfo<'a> {
    replacements: Replacements<'a>,
    start: &'a str,
}

fn parse_input(input: &str) -> InputInfo {
    let mut secs = input.split("\n\n");
    let replacements_sec = secs.next().unwrap();

    let mut replacements = HashMap::new();
    for line in replacements_sec.lines() {
        let mut spl = line.split(" => ");
        let source = spl.next().unwrap();
        let target = spl.next().unwrap();

        let e = replacements.entry(source).or_insert(vec![]);
        e.push(target);
    }

    let start = secs.next().unwrap();

    InputInfo {
        replacements,
        start,
    }
}

fn one_replacement(input_info: &InputInfo) -> usize {
    let mut results = HashSet::new();
    for i in 0..input_info.start.len() {
        let prefix = &input_info.start[0..i];
        let slice = &input_info.start[i..];
        for (source, targets) in input_info.replacements.iter() {
            if slice.starts_with(source) {
                let suffix_offset = i + source.len();
                let suffix = &input_info.start[suffix_offset..];
                for target in targets {
                    let repl_str = format!("{}{}{}", prefix, target, suffix);
                    results.insert(repl_str);
                }
            }
        }
    }
    results.len()
}

impl Day for Day19 {
    fn star1(&self, input: &str) -> String {
        let input_info = parse_input(input);
        format!("{}", one_replacement(&input_info))
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day19 {};
        let input = r#"H => HO
H => OH
O => HH

HOH"#;
        assert_eq!(d.star1(input), "4");
    }
}
