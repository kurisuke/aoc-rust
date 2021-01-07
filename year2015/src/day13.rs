use common::day::Day;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Day13 {}

type Relations<'a> = HashMap<(&'a str, &'a str), i64>;

fn parse_input(input: &str) -> Relations {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            let target = parts[0];
            let source = parts[parts.len() - 1];
            let source = &source[0..source.len() - 1]; // without the final .
            let sign = match parts[2] {
                "gain" => 1,
                "lose" => -1,
                _ => {
                    panic!("Cannot parse line: {}", line);
                }
            };
            let diff = sign * parts[3].parse::<i64>().unwrap();
            ((target, source), diff)
        })
        .collect()
}

fn best_happiness(relations: &Relations) -> i64 {
    let mut persons: HashSet<_> = relations.keys().map(|x| x.0).collect();
    let num_persons = persons.len();
    let first_person = *persons.iter().next().unwrap();
    persons.remove(first_person);

    let mut best_happiness = std::i64::MIN;
    for mut seating in persons.iter().permutations(num_persons - 1) {
        seating.push(&first_person);
        let mut happiness = 0;
        for (i, seat) in seating.iter().enumerate() {
            let neighbor_left = seating[(i + num_persons - 1) % num_persons];
            let neighbor_right = seating[(i + num_persons + 1) % num_persons];
            happiness +=
                relations[&(**seat, *neighbor_left)] + relations[&(**seat, *neighbor_right)];
        }
        best_happiness = best_happiness.max(happiness);
    }
    best_happiness
}

fn add_self(relations: &mut Relations) {
    let persons: HashSet<_> = relations.keys().map(|x| x.0).collect();
    for person in persons {
        relations.insert((person, "Self"), 0);
        relations.insert(("Self", person), 0);
    }
}

impl Day for Day13 {
    fn star1(&self, input: &str) -> String {
        let relations = parse_input(input);
        format!("{}", best_happiness(&relations))
    }

    fn star2(&self, input: &str) -> String {
        let mut relations = parse_input(input);
        add_self(&mut relations);
        format!("{}", best_happiness(&relations))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let d = Day13 {};
        let input = r#"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."#;
        assert_eq!(d.star1(input), "330");
    }
}
