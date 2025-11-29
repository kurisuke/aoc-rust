use common::day::Day;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct Day07 {}

struct Node<'a> {
    weight: usize,
    children: Vec<&'a str>,
}

type Discs<'a> = HashMap<&'a str, Node<'a>>;

fn tree_weight(discs: &Discs, root_node_name: &str) -> usize {
    let root_node = discs.get(root_node_name).unwrap();
    root_node.weight
        + root_node
            .children
            .iter()
            .map(|c| tree_weight(discs, c))
            .sum::<usize>()
}

fn find_wrong_weight(discs: &Discs, root_node_name: &str, weight_diff: isize) -> usize {
    let root_node = discs.get(root_node_name).unwrap();
    let child_weights: Vec<_> = root_node
        .children
        .iter()
        .map(|c| tree_weight(discs, c))
        .collect();

    // Find the one subtree which weighs less than the others.
    // "Odd one out" assumes there are at least 3 elements.
    assert!(child_weights.len() >= 3);
    let mut odd_one_out = None;
    for i in 0..child_weights.len() {
        let prev = (i + child_weights.len() - 1) % child_weights.len();
        let next = (i + 1) % child_weights.len();
        if child_weights[i] != child_weights[prev] && child_weights[i] != child_weights[next] {
            odd_one_out = Some(i);
            break;
        }
    }
    if let Some(i) = odd_one_out {
        // Children are unbalanced. We have identified the unbalanced subtree and continue there.
        let weight_diff =
            child_weights[(i + 1) % child_weights.len()] as isize - child_weights[i] as isize;
        find_wrong_weight(discs, root_node.children[i], weight_diff)
    } else {
        // Children are balanced, so the problem is in this node.
        (root_node.weight as isize + weight_diff) as usize
    }
}

fn parse_input(input: &str) -> Discs<'_> {
    let re = Regex::new(r"(\w+) \((\d+)\)").unwrap();
    input
        .lines()
        .map(|line| {
            let mut it = line.split(" -> ");
            let first = it.next().unwrap();
            let caps = re.captures(first).unwrap();
            let name = caps.get(1).unwrap().as_str();
            let weight = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let children: Vec<_> = match it.next() {
                Some(w) => w.split(", ").collect(),
                None => vec![],
            };
            (name, Node { weight, children })
        })
        .collect()
}

impl Day for Day07 {
    fn star1(&self, input: &str) -> String {
        let discs = parse_input(input);
        let all_nodes: HashSet<_> = discs.keys().collect();
        let child_nodes: HashSet<_> = discs.values().flat_map(|v| v.children.iter()).collect();
        (&all_nodes - &child_nodes)
            .iter()
            .next()
            .unwrap()
            .to_string()
    }

    fn star2(&self, input: &str) -> String {
        let discs = parse_input(input);
        let all_nodes: HashSet<_> = discs.keys().collect();
        let child_nodes: HashSet<_> = discs.values().flat_map(|v| v.children.iter()).collect();
        let root_node_name = *(&all_nodes - &child_nodes).iter().next().unwrap();
        // print_tree_weights(&discs, root_node_name);

        format!("{}", find_wrong_weight(&discs, root_node_name, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day07 {};
        let input = r#"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"#;
        assert_eq!(d.star1(input), "tknk");
        assert_eq!(d.star2(input), "60");
    }
}
