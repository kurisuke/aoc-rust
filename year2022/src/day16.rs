use common::day::Day;
use itertools::Itertools;
use regex::Regex;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

pub struct Day16 {}

type ValveLabel = u8;
type Valves = Vec<Valve>;
type Distances = HashMap<(ValveLabel, ValveLabel), usize>;
type Path = Vec<(ValveLabel, usize)>;

#[derive(Eq, PartialEq, Hash)]
struct Valve {
    flow_rate: usize,
    tunnels: Vec<ValveLabel>,
}

fn shortest_paths(valves: &Valves, from: ValveLabel) -> Distances {
    let mut targets: HashSet<_> = valves
        .iter()
        .enumerate()
        .filter(|(n, v)| v.flow_rate > 0 && *n != from.into())
        .map(|(n, _)| n as u8)
        .collect();

    let mut paths = HashMap::new();
    let mut visited = HashSet::new();
    let mut next_queue = VecDeque::new();
    next_queue.push_back((from, 0));

    while let Some((pos, steps)) = next_queue.pop_front() {
        visited.insert(pos);

        if targets.contains(&pos) {
            paths.insert((from, pos), steps);
            targets.remove(&pos);

            if targets.is_empty() {
                break;
            }
        }

        for next in valves[pos as usize].tunnels.iter() {
            if !visited.contains(next) {
                next_queue.push_back((*next, steps + 1));
            }
        }
    }

    paths
}

fn find_paths(distances: &Distances, targets: &HashSet<ValveLabel>, path_now: Path) -> Vec<Path> {
    let mut paths = vec![];
    let (pos, time_limit) = path_now.last().unwrap();

    for target in targets.iter() {
        let distance = distances.get(&(*pos, *target)).unwrap();
        if distance + 2 < *time_limit {
            let mut targets_new = targets.clone();
            let mut path_new = path_now.clone();
            targets_new.remove(target);
            path_new.push((*target, time_limit - distance - 1));
            paths.extend(find_paths(distances, &targets_new, path_new));
        }
    }

    if paths.is_empty() {
        vec![path_now]
    } else {
        paths
    }
}

fn total_flow(valves: &Valves, path: &Path) -> usize {
    let mut total_flow = 0;
    for (valve_label, minutes_open) in path.iter().skip(1) {
        let flow_rate = valves[*valve_label as usize].flow_rate;
        total_flow += flow_rate * minutes_open;
    }
    total_flow
}

fn parse_input(input: &str) -> (Valves, ValveLabel) {
    let map_labels: HashMap<&str, ValveLabel> = input
        .lines()
        .enumerate()
        .map(|(i, line)| (line.split_whitespace().nth(1).unwrap(), i as u8))
        .collect();

    let re = Regex::new(
        r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnel[s]? lead[s]? to valve[s]? ([A-Z,\s]+)$",
    )
    .unwrap();
    (
        input
            .lines()
            .map(|line| {
                let captures = re.captures(line).unwrap();
                let flow_rate = captures[2].parse().unwrap();
                let tunnels = captures[3]
                    .split(", ")
                    .map(|s| *map_labels.get(s).unwrap())
                    .collect();
                Valve { flow_rate, tunnels }
            })
            .collect(),
        *map_labels.get("AA").unwrap(),
    )
}

fn prepare(input: &str) -> (Valves, Distances, HashSet<ValveLabel>, ValveLabel) {
    let (valves, start) = parse_input(input);

    let mut distances = shortest_paths(&valves, start);

    let targets: HashSet<_> = valves
        .iter()
        .enumerate()
        .filter(|(_, v)| v.flow_rate > 0)
        .map(|(n, _)| n as u8)
        .collect();

    for from in targets.iter() {
        distances.extend(shortest_paths(&valves, *from as u8));
    }

    (valves, distances, targets, start)
}

impl Day for Day16 {
    fn star1(&self, input: &str) -> String {
        let (valves, distances, targets, start) = prepare(input);

        let paths = find_paths(&distances, &targets, vec![(start, 30)]);

        let max_total_flow = paths
            .iter()
            .map(|path| total_flow(&valves, path))
            .max()
            .unwrap();
        format!("{}", max_total_flow)
    }

    fn star2(&self, input: &str) -> String {
        let (valves, distances, targets, start) = prepare(input);

        let paths = find_paths(&distances, &targets, vec![(start, 26)]);
        let mut best_scores = HashMap::new();

        for mut path in paths {
            while path.len() > 1 {
                let total_flow_first = total_flow(&valves, &path);
                let opened_valves: BTreeSet<_> = path.iter().map(|(valve, _)| *valve).collect();
                let entry = best_scores.entry(opened_valves).or_insert(total_flow_first);
                if total_flow_first > *entry {
                    *entry = total_flow_first;
                }
                path.pop();
            }
        }

        let mut total_flow_max = 0;
        for c in best_scores.iter().combinations(2) {
            let (valves1, score1) = c[0];
            let (valves2, score2) = c[1];

            // ensure only common node is the start (AA)
            if valves1.intersection(valves2).count() == 1 {
                total_flow_max = total_flow_max.max(score1 + score2);
            }
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
