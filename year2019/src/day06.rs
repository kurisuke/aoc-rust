use common::day::Day;
use std::collections::HashMap;

pub struct Day06 {}

type Orbits<'a> = HashMap<&'a str, &'a str>;

fn parse_input(input: &str) -> Orbits<'_> {
    input
        .lines()
        .map(|l| {
            let (parent, child) = l.split_once(')').unwrap();
            (child, parent)
        })
        .collect()
}

fn count_orbits(orbits: &Orbits) -> usize {
    let mut sum_orbits = 0;
    for mut child in orbits.keys() {
        let mut parent = orbits.get(child).unwrap();
        let mut n = 1;
        while parent != &"COM" {
            child = parent;
            parent = orbits.get(child).unwrap();
            n += 1;
        }
        sum_orbits += n;
    }
    sum_orbits
}

fn ancestors<'a>(orbits: &'a Orbits, obj: &'a str) -> Vec<&'a str> {
    let mut anc = vec![];
    let mut child = obj;
    loop {
        let parent = orbits.get(child).unwrap();
        anc.push(*parent);
        if parent == &"COM" {
            break;
        }
        child = parent;
    }
    anc
}

fn count_transfers(orbits: &Orbits, obj1: &str, obj2: &str) -> usize {
    let anc1 = ancestors(orbits, obj1);
    let anc2 = ancestors(orbits, obj2);

    let (t1, a1) = anc1
        .iter()
        .enumerate()
        .find(|(_, a1)| anc2.contains(a1))
        .unwrap();
    let t2 = anc2
        .iter()
        .enumerate()
        .find(|(_, a2)| *a2 == a1)
        .map(|(t2, _)| t2)
        .unwrap();
    t1 + t2
}

impl Day for Day06 {
    fn star1(&self, input: &str) -> String {
        let orbits = parse_input(input);
        format!("{}", count_orbits(&orbits))
    }

    fn star2(&self, input: &str) -> String {
        let orbits = parse_input(input);
        format!("{}", count_transfers(&orbits, "YOU", "SAN"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#;

        let d = Day06 {};
        assert_eq!(d.star1(input), "42");
    }

    #[test]
    fn ex2() {
        let input = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"#;

        let d = Day06 {};
        assert_eq!(d.star2(input), "4");
    }
}
