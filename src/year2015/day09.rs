use crate::day::Day;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day09 {}

type Distances<'a> = HashMap<(&'a str, &'a str), usize>;

fn parse_input(input: &str) -> Distances {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(" = ");
            let locs = it.next().unwrap();
            let distance = it.next().unwrap().parse::<usize>().unwrap();
            let locs: Vec<_> = locs.split(" to ").sorted().collect();
            ((locs[0], locs[1]), distance)
        })
        .collect()
}

fn find_shortest(distances: &Distances) -> usize {
    let locations1: HashSet<_> = distances.keys().map(|x| x.0).collect();
    let locations2: HashSet<_> = distances.keys().map(|x| x.1).collect();
    let locations: HashSet<_> = locations1.union(&locations2).collect();
    let mut best_dist = std::usize::MAX;
    for perm in locations.iter().permutations(locations.len()) {
        let mut dist = 0;
        for places in perm.windows(2) {
            let places: Vec<_> = places.iter().sorted().collect();
            dist += distances[&(***places[0], ***places[1])];
        }
        best_dist = best_dist.min(dist);
    }
    best_dist
}

fn find_longest(distances: &Distances) -> usize {
    let locations1: HashSet<_> = distances.keys().map(|x| x.0).collect();
    let locations2: HashSet<_> = distances.keys().map(|x| x.1).collect();
    let locations: HashSet<_> = locations1.union(&locations2).collect();
    let mut best_dist = std::usize::MIN;
    for perm in locations.iter().permutations(locations.len()) {
        let mut dist = 0;
        for places in perm.windows(2) {
            let places: Vec<_> = places.iter().sorted().collect();
            dist += distances[&(***places[0], ***places[1])];
        }
        best_dist = best_dist.max(dist);
    }
    best_dist
}

impl Day for Day09 {
    fn star1(&self, input: &str) -> String {
        let distances = parse_input(input);
        format!("{}", find_shortest(&distances))
    }

    fn star2(&self, input: &str) -> String {
        let distances = parse_input(input);
        format!("{}", find_longest(&distances))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#;
        let d = Day09 {};
        assert_eq!(d.star1(input), "605");
        assert_eq!(d.star2(input), "982");
    }
}
