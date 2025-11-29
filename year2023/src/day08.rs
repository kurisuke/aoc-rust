use std::collections::HashMap;

use common::day::Day;
use util::gcd::lcm;

pub struct Day08 {}

impl Day for Day08 {
    fn star1(&self, input: &str) -> String {
        let (instructions, nodes) = parse_input(input);
        traverse(&instructions, &nodes, "AAA", "ZZZ").to_string()
    }

    fn star2(&self, input: &str) -> String {
        let (instructions, nodes) = parse_input(input);

        let steps_single_paths: Vec<_> = nodes
            .keys()
            .filter(|n| n.ends_with('A'))
            .map(|a_node| traverse_pt2(&instructions, &nodes, a_node, "Z"))
            .collect();

        steps_single_paths
            .into_iter()
            .reduce(lcm)
            .unwrap()
            .to_string()
    }
}

type Nodes<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse_input(input: &str) -> (Vec<char>, Nodes<'_>) {
    let (instructions, rest) = input.split_once("\n\n").unwrap();
    let instructions = instructions.chars().collect();
    (instructions, parse_nodes(rest))
}

fn parse_nodes(input: &str) -> Nodes<'_> {
    input
        .lines()
        .map(|line| {
            let (node, rest) = line.split_once(" = ").unwrap();
            let rest = rest.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
            let (left, right) = rest.split_once(", ").unwrap();
            (node, (left, right))
        })
        .collect()
}

fn traverse(instructions: &[char], nodes: &Nodes, start: &str, end: &str) -> usize {
    let mut steps = 0;
    let mut cur = start;
    while cur != end {
        cur = match instructions[steps % instructions.len()] {
            'L' => nodes[cur].0,
            'R' => nodes[cur].1,
            _ => unreachable!(),
        };
        steps += 1;
    }
    steps
}

fn traverse_pt2(instructions: &[char], nodes: &Nodes, start: &str, ends_with: &str) -> usize {
    let mut steps = 0;
    let mut cur = start;
    while !cur.ends_with(ends_with) {
        cur = match instructions[steps % instructions.len()] {
            'L' => nodes[cur].0,
            'R' => nodes[cur].1,
            _ => unreachable!(),
        };
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

        let d = Day08 {};
        assert_eq!(d.star1(input), "2");
    }

    #[test]
    fn ex2() {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

        let d = Day08 {};
        assert_eq!(d.star1(input), "6");
    }

    #[test]
    fn ex3() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

        let d = Day08 {};
        assert_eq!(d.star2(input), "6");
    }
}
