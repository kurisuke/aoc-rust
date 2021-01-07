use common::day::Day;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

pub struct Day12 {}

type Pipes = HashMap<usize, Vec<usize>>;

fn connected(pipes: &Pipes, n: usize) -> BTreeSet<usize> {
    let mut visited = BTreeSet::new();
    let mut frontier = VecDeque::new();
    frontier.push_back(n);
    visited.insert(n);
    while let Some(cur) = frontier.pop_front() {
        for next in pipes.get(&cur).unwrap() {
            if visited.insert(*next) {
                frontier.push_back(*next);
            }
        }
    }
    visited
}

fn parse_input(input: &str) -> HashMap<usize, Vec<usize>> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(" <-> ");
            let first = it.next().unwrap();
            let second = it.next().unwrap();

            let from = first.parse::<usize>().unwrap();
            let to = second
                .split(", ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            (from, to)
        })
        .collect()
}

impl Day for Day12 {
    fn star1(&self, input: &str) -> String {
        let pipes = parse_input(input);
        let conn = connected(&pipes, 0);
        format!("{}", conn.len())
    }

    fn star2(&self, input: &str) -> String {
        let pipes = parse_input(input);
        let mut groups = HashSet::new();
        for start in pipes.keys() {
            groups.insert(connected(&pipes, *start));
        }
        format!("{}", groups.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day12 {};
        let input = r#"0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5"#;
        assert_eq!(d.star1(input), "6");
        assert_eq!(d.star2(input), "2");
    }
}
