use common::day::Day;
use regex::Regex;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

pub struct Day16 {}

type Valves = HashMap<String, Valve>;
type Distances = HashMap<(String, String), usize>;
type Path = Vec<(String, usize)>;

struct Valve {
    flow_rate: usize,
    tunnels: Vec<String>,
}

fn shortest_paths(valves: &Valves, from: &str) -> Distances {
    let mut targets: HashSet<_> = valves
        .iter()
        .filter(|(n, v)| v.flow_rate > 0 && *n != from)
        .map(|(n, _)| n.as_str())
        .collect();

    let mut paths = HashMap::new();
    let mut visited = HashSet::new();
    let mut next_queue = VecDeque::new();
    next_queue.push_back((from, 0));

    while let Some((pos, steps)) = next_queue.pop_front() {
        visited.insert(pos);

        if targets.contains(pos) {
            paths.insert((from.to_string(), pos.to_string()), steps);
            targets.remove(pos);

            if targets.is_empty() {
                break;
            }
        }

        for next in valves.get(pos).unwrap().tunnels.iter() {
            if !visited.contains(next.as_str()) {
                next_queue.push_back((next, steps + 1));
            }
        }
    }

    paths
}

fn find_paths(distances: &Distances, targets: HashSet<String>, path_now: Path) -> Vec<Path> {
    if targets.is_empty() {
        vec![path_now]
    } else {
        let mut paths = vec![];
        let (pos, time_limit) = path_now.last().unwrap();

        for target in targets.iter() {
            let distance = distances
                .get(&(pos.to_string(), target.to_string()))
                .unwrap();
            if distance + 1 < *time_limit {
                let mut targets_new = targets.clone();
                let mut path_new = path_now.clone();
                targets_new.remove(target.as_str());
                path_new.push((target.clone(), time_limit - distance - 1));
                paths.extend(find_paths(distances, targets_new, path_new));
            }
        }

        if paths.is_empty() {
            vec![path_now]
        } else {
            paths
        }
    }
}

fn total_flow(valves: &Valves, path: &Path) -> usize {
    let mut total_flow = 0;
    for (valve_label, minutes_open) in path.iter().skip(1) {
        let flow_rate = valves.get(valve_label).unwrap().flow_rate;
        total_flow += flow_rate * minutes_open;
    }
    total_flow
}

fn parse_input(input: &str) -> Valves {
    let re = Regex::new(
        r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? ([A-Z,\s]+)$",
    )
    .unwrap();
    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            let name = &captures[1];
            let flow_rate = captures[2].parse().unwrap();
            let tunnels = captures[3].split(", ").map(|s| s.to_string()).collect();
            (name.to_string(), Valve { flow_rate, tunnels })
        })
        .collect()
}

impl Day for Day16 {
    fn star1(&self, input: &str) -> String {
        let valves = parse_input(input);
        assert_eq!(input.lines().count(), valves.len());

        let mut distances = shortest_paths(&valves, "AA");

        let targets: HashSet<_> = valves
            .iter()
            .filter(|(_, v)| v.flow_rate > 0)
            .map(|(n, _)| n.to_string())
            .collect();

        for from in targets.iter() {
            distances.extend(shortest_paths(&valves, from));
        }

        for ((src, dest), steps) in distances.iter() {
            if src != "AA" {
                let reverse = (dest.to_string(), src.to_string());
                assert_eq!(steps, distances.get(&reverse).unwrap());
            }
        }

        let paths = find_paths(&distances, targets, vec![(String::from("AA"), 30)]);

        let max_total_flow = paths
            .iter()
            .map(|path| total_flow(&valves, path))
            .max()
            .unwrap();
        format!("{}", max_total_flow)
    }

    fn star2(&self, input: &str) -> String {
        let valves = parse_input(input);
        assert_eq!(input.lines().count(), valves.len());

        let mut distances = shortest_paths(&valves, "AA");

        let targets: HashSet<_> = valves
            .iter()
            .filter(|(_, v)| v.flow_rate > 0)
            .map(|(n, _)| n.to_string())
            .collect();

        for from in targets.iter() {
            distances.extend(shortest_paths(&valves, from));
        }

        for ((src, dest), steps) in distances.iter() {
            if src != "AA" {
                let reverse = (dest.to_string(), src.to_string());
                assert_eq!(steps, distances.get(&reverse).unwrap());
            }
        }

        let paths = find_paths(&distances, targets.clone(), vec![(String::from("AA"), 26)]);
        let mut best_scores = HashMap::new();

        for mut path in paths {
            while path.len() > 1 {
                let total_flow_first = total_flow(&valves, &path);
                let opened_valves: BTreeSet<_> =
                    path.iter().map(|(valve, _)| valve.to_string()).collect();
                let entry = best_scores.entry(opened_valves).or_insert(total_flow_first);
                if total_flow_first > *entry {
                    *entry = total_flow_first;
                }
                path.pop();
            }
        }

        let mut total_flow_max = 0;
        for (opened_valves, total_flow_first) in best_scores.iter() {
            let mut left_valves = targets.clone();
            for valve in opened_valves.iter() {
                left_valves.remove(valve);
            }

            let paths_second = find_paths(&distances, left_valves, vec![(String::from("AA"), 26)]);
            let total_flow_second = paths_second
                .iter()
                .map(|path_second| total_flow(&valves, path_second))
                .max()
                .unwrap();
            total_flow_max = total_flow_max.max(total_flow_first + total_flow_second);
        }

        format!("{}", total_flow_max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

        let d = Day16 {};
        assert_eq!(d.star1(input), "1651");
        assert_eq!(d.star2(input), "1707");
    }
}
