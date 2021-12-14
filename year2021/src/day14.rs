use common::day::Day;

use std::collections::HashMap;
use util::chardistrib;

type Rules = HashMap<(char, char), char>;

struct State {
    pub pairs: HashMap<(char, char), usize>,
    pub distrib: HashMap<char, usize>,
}

fn parse_input(input: &str) -> (State, Rules) {
    let init_str = input.split("\n\n").next().unwrap();
    let mut pairs = HashMap::new();
    for w in init_str.chars().collect::<Vec<_>>().windows(2) {
        *pairs.entry((w[0], w[1])).or_insert(0) += 1;
    }
    let distrib = chardistrib::char_distribution(init_str);
    let init = State { pairs, distrib };

    let rules_str = input.split("\n\n").nth(1).unwrap();
    let mut rules = HashMap::new();
    for l in rules_str.lines() {
        let spl: Vec<_> = l.split(" -> ").collect();
        let in_chars: Vec<_> = spl[0].chars().collect();
        rules.insert((in_chars[0], in_chars[1]), spl[1].chars().next().unwrap());
    }
    (init, rules)
}

fn step(state: &State, rules: &Rules) -> State {
    let mut new_distrib = state.distrib.clone();
    let mut new_pairs = HashMap::new();

    for (k, v) in state.pairs.iter() {
        if let Some(new_char) = rules.get(k) {
            *new_pairs.entry((k.0, *new_char)).or_insert(0) += v;
            *new_pairs.entry((*new_char, k.1)).or_insert(0) += v;
            *new_distrib.entry(*new_char).or_insert(0) += v;
        } else {
            *new_pairs.entry(*k).or_insert(0) += v;
        }
    }
    State {
        pairs: new_pairs,
        distrib: new_distrib,
    }
}

fn run_steps(input: &str, i: usize) -> usize {
    let (mut state, rules) = parse_input(input);
    for _ in 0..i {
        state = step(&state, &rules);
    }
    let max_v = state.distrib.values().max().unwrap();
    let min_v = state.distrib.values().min().unwrap();
    max_v - min_v
}

pub struct Day14 {}

impl Day for Day14 {
    fn star1(&self, input: &str) -> String {
        format!("{}", run_steps(input, 10))
    }

    fn star2(&self, input: &str) -> String {
        format!("{}", run_steps(input, 40))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

        let d = Day14 {};
        assert_eq!(d.star1(input), "1588");
    }
}
