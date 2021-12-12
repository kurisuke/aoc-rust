use common::day::Day;

use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day12 {}

type Connections<'a> = HashMap<&'a str, HashSet<&'a str>>;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Path<'a> {
    pub path: Vec<&'a str>,
    pub single_visited: Option<&'a str>,
}

fn parse_input(input: &str) -> Connections {
    let mut connections = HashMap::new();
    for l in input.lines() {
        let spl: Vec<_> = l.split('-').collect();

        let e = connections.entry(spl[0]).or_insert_with(HashSet::new);
        e.insert(spl[1]);

        let e = connections.entry(spl[1]).or_insert_with(HashSet::new);
        e.insert(spl[0]);
    }
    connections
}

fn find_paths(connections: &Connections, repeat: bool) -> usize {
    let mut finished = 0;
    let mut in_progress = VecDeque::new();
    in_progress.push_back(Path {
        path: vec!["start"],
        single_visited: None,
    });

    while let Some(p) = in_progress.pop_front() {
        let current = p.path.last().unwrap();
        for next in connections.get(current).unwrap() {
            if *next == "start" {
                continue;
            } else if *next == "end" {
                finished += 1;
            } else if next.chars().all(|c| c.is_lowercase()) {
                // small cave
                if !p.path.contains(next) {
                    let mut next_path = p.clone();
                    next_path.path.push(next);
                    in_progress.push_back(next_path);
                } else if repeat && p.single_visited.is_none() {
                    let mut next_path = p.clone();
                    next_path.path.push(next);
                    next_path.single_visited = Some(current);
                    in_progress.push_back(next_path);
                }
            } else {
                // large cave
                let mut next_path = p.clone();
                next_path.path.push(next);
                in_progress.push_back(next_path);
            }
        }
    }

    finished
}

impl Day for Day12 {
    fn star1(&self, input: &str) -> String {
        let connections = parse_input(input);
        let paths = find_paths(&connections, false);
        format!("{}", paths)
    }

    fn star2(&self, input: &str) -> String {
        let connections = parse_input(input);
        let paths = find_paths(&connections, true);
        format!("{}", paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

        let d = Day12 {};
        assert_eq!(d.star1(input), "10");
        assert_eq!(d.star2(input), "36");
    }

    #[test]
    fn ex2() {
        let input = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;

        let d = Day12 {};
        assert_eq!(d.star1(input), "19");
        assert_eq!(d.star2(input), "103");
    }

    #[test]
    fn ex3() {
        let input = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;

        let d = Day12 {};
        assert_eq!(d.star1(input), "226");
        assert_eq!(d.star2(input), "3509");
    }
}
