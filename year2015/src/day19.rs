use common::day::Day;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct Day19 {}

type Replacements<'a> = HashMap<&'a str, Vec<&'a str>>;

struct InputInfo<'a> {
    replacements: Replacements<'a>,
    els: Vec<&'a str>,
}

fn parse_input(input: &str) -> InputInfo {
    let mut secs = input.split("\n\n");
    let replacements_sec = secs.next().unwrap();

    let mut replacements = HashMap::new();
    for line in replacements_sec.lines() {
        let mut spl = line.split(" => ");
        let source = spl.next().unwrap();
        let target = spl.next().unwrap();

        let e = replacements.entry(source).or_insert_with(Vec::new);
        e.push(target);
    }

    let el_re = Regex::new(r"[A-Z][a-z]*").unwrap();
    let els: Vec<_> = el_re
        .find_iter(secs.next().unwrap())
        .map(|x| x.as_str())
        .collect();

    InputInfo { replacements, els }
}

fn one_replacement(input_info: &InputInfo) -> usize {
    let mut results = HashSet::new();
    for (i, el) in input_info.els.iter().enumerate() {
        let prefix = input_info.els[..i].iter().fold(String::new(), |a, b| a + b);
        let suffix: String = input_info.els[i + 1..]
            .iter()
            .fold(String::new(), |a, b| a + b);
        if let Some(targets) = input_info.replacements.get(el) {
            for target in targets {
                let repl_str = format!("{}{}{}", prefix, target, suffix);
                results.insert(repl_str);
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

    fn star2(&self, input: &str) -> String {
        let input_info = parse_input(input);
        let count_rn = input_info.els.iter().filter(|x| **x == "Rn").count();
        let count_y = input_info.els.iter().filter(|x| **x == "Y").count();
        format!("{}", input_info.els.len() - 2 * count_rn - 2 * count_y - 1)
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
