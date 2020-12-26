use crate::day::Day;
use std::collections::HashMap;

pub struct Day16 {}

type Compounds<'a> = HashMap<&'a str, usize>;

fn parse_input(input: &str) -> Vec<Compounds> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let prefix = format!("Sue {}: ", i + 1);
            let compounds_str = &line[prefix.len()..];
            let compounds: HashMap<_, _> = compounds_str
                .split(", ")
                .map(|compound| {
                    let mut it = compound.split(": ");
                    let name = it.next().unwrap();
                    let number = it.next().unwrap().parse::<usize>().unwrap();
                    (name, number)
                })
                .collect();
            compounds
        })
        .collect()
}

fn match_clues(clues: &Compounds, knowledge: &Compounds) -> bool {
    knowledge.iter().all(|(k, v)| clues.get(k).unwrap() == v)
}

fn match_clues_star2(clues: &Compounds, knowledge: &Compounds) -> bool {
    knowledge.iter().all(|(k, v)| match k {
        &"cats" | &"trees" => v > clues.get(k).unwrap(),
        &"pomeranians" | &"goldfish" => v < clues.get(k).unwrap(),
        _ => v == clues.get(k).unwrap(),
    })
}

impl Day for Day16 {
    fn star1(&self, input: &str) -> String {
        let mut clues = HashMap::new();
        clues.insert("children", 3);
        clues.insert("cats", 7);
        clues.insert("samoyeds", 2);
        clues.insert("pomeranians", 3);
        clues.insert("akitas", 0);
        clues.insert("vizslas", 0);
        clues.insert("goldfish", 5);
        clues.insert("trees", 3);
        clues.insert("cars", 2);
        clues.insert("perfumes", 1);

        let aunts = parse_input(input);
        let solutions: Vec<_> = aunts
            .iter()
            .enumerate()
            .filter(|(_, aunt)| match_clues(&clues, aunt))
            .collect();
        format!("{}", solutions[0].0 + 1)
    }

    fn star2(&self, input: &str) -> String {
        let mut clues = HashMap::new();
        clues.insert("children", 3);
        clues.insert("cats", 7);
        clues.insert("samoyeds", 2);
        clues.insert("pomeranians", 3);
        clues.insert("akitas", 0);
        clues.insert("vizslas", 0);
        clues.insert("goldfish", 5);
        clues.insert("trees", 3);
        clues.insert("cars", 2);
        clues.insert("perfumes", 1);

        let aunts = parse_input(input);
        let solutions: Vec<_> = aunts
            .iter()
            .enumerate()
            .filter(|(_, aunt)| match_clues_star2(&clues, aunt))
            .collect();
        format!("{}", solutions[0].0 + 1)
    }
}
