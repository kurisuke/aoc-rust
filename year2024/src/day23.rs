use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use common::day::Day;
use itertools::Itertools;

pub struct Day23 {}

impl Day for Day23 {
    fn star1(&self, input: &str) -> String {
        let connections = parse_input(input);

        let mut found = HashSet::new();
        for (c1_key, c1_connections) in connections.iter().filter(|c| c.0 .0 == 't') {
            for c2_key in c1_connections {
                for c3_key in connections.get(c2_key).unwrap() {
                    if c1_key != c3_key && c1_connections.contains(c3_key) {
                        let mut set = [c1_key, c2_key, c3_key];
                        set.sort_unstable();
                        found.insert(set);
                    }
                }
            }
        }
        found.len().to_string()
    }

    fn star2(&self, input: &str) -> String {
        let connections = parse_input(input);

        let max_set = connections
            .keys()
            .map(|start| find_connected(&connections, start))
            .max_by_key(|c| c.len())
            .unwrap();

        max_set.iter().map(|c| format!("{}{}", c.0, c.1)).join(",")
    }
}

type Computer = (char, char);

fn parse_input(input: &str) -> HashMap<Computer, HashSet<Computer>> {
    let mut connections = HashMap::new();
    for line in input.lines() {
        let parts: Vec<_> = line.split('-').collect();
        let c1 = (
            parts[0].chars().next().unwrap(),
            parts[0].chars().nth(1).unwrap(),
        );
        let c2 = (
            parts[1].chars().next().unwrap(),
            parts[1].chars().nth(1).unwrap(),
        );
        let e = connections.entry(c1).or_insert(HashSet::new());
        e.insert(c2);
        let e = connections.entry(c2).or_insert(HashSet::new());
        e.insert(c1);
    }
    connections
}

fn find_connected(
    connections: &HashMap<Computer, HashSet<Computer>>,
    start: &Computer,
) -> BTreeSet<Computer> {
    let mut connected = BTreeSet::new();

    let mut search_nodes = VecDeque::new();
    search_nodes.push_back(*start);

    while let Some(cur_node) = search_nodes.pop_front() {
        if connected
            .iter()
            .all(|node| connections.get(node).unwrap().contains(&cur_node))
        {
            connected.insert(cur_node);
            for next_node in connections.get(&cur_node).unwrap() {
                if !connected.contains(next_node) {
                    search_nodes.push_back(*next_node);
                }
            }
        }
    }

    connected
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn"#;

    #[test]
    fn star1() {
        let d = Day23 {};
        assert_eq!(d.star1(INPUT), "7");
    }

    #[test]
    fn star2() {
        let d = Day23 {};
        assert_eq!(d.star2(INPUT), "co,de,ka,ta");
    }
}
