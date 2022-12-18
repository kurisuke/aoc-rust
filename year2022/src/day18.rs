use common::day::Day;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day18 {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coords3D(i16, i16, i16);

impl Coords3D {
    fn parse(line: &str) -> Coords3D {
        let mut it = line.split(',');
        Coords3D(
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        )
    }

    fn dist(&self, other: &Self) -> i16 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Coords3D> + '_ {
    input.lines().map(Coords3D::parse)
}

fn surface_area(cubes: &[Coords3D]) -> u16 {
    let mut surfaces_exposed: HashMap<_, _> = cubes.iter().map(|c| (c, 6)).collect();

    for c in cubes.iter().combinations(2) {
        let c1 = c[0];
        let c2 = c[1];

        if c1.dist(c2) == 1 {
            let e1 = surfaces_exposed.entry(c1).or_insert(6);
            *e1 -= 1;
            let e2 = surfaces_exposed.entry(c2).or_insert(6);
            *e2 -= 1;
        }
    }
    surfaces_exposed.values().sum()
}

fn flood_outside_area(cubes: &[Coords3D]) -> u16 {
    let (x_min, x_max, y_min, y_max, z_min, z_max) = cubes.iter().fold(
        (i16::MAX, i16::MIN, i16::MAX, i16::MIN, i16::MAX, i16::MIN),
        |a, c| {
            (
                a.0.min(c.0),
                a.1.max(c.0),
                a.2.min(c.1),
                a.3.max(c.1),
                a.4.min(c.2),
                a.5.max(c.2),
            )
        },
    );

    let (x_min, x_max, y_min, y_max, z_min, z_max) = (
        x_min - 1,
        x_max + 1,
        y_min - 1,
        y_max + 1,
        z_min - 1,
        z_max + 1,
    );

    let cubes_set: HashSet<_> = cubes.iter().collect();

    let mut queue = VecDeque::new();
    let start = Coords3D(x_min, y_min, z_min);
    assert!(!cubes_set.contains(&start));
    queue.push_back(start);

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut surface_cubes = HashMap::new();

    while let Some(pos) = queue.pop_front() {
        for neighbor in [
            Coords3D(pos.0 - 1, pos.1, pos.2),
            Coords3D(pos.0 + 1, pos.1, pos.2),
            Coords3D(pos.0, pos.1 - 1, pos.2),
            Coords3D(pos.0, pos.1 + 1, pos.2),
            Coords3D(pos.0, pos.1, pos.2 - 1),
            Coords3D(pos.0, pos.1, pos.2 + 1),
        ] {
            if !visited.contains(&neighbor)
                && pos.0 >= x_min
                && pos.0 <= x_max
                && pos.1 >= y_min
                && pos.1 <= y_max
                && pos.2 >= z_min
                && pos.2 <= z_max
            {
                if cubes_set.contains(&neighbor) {
                    // count from how many neighboring cubes we can reach this
                    // this is the number of surfaces
                    let e = surface_cubes.entry(neighbor).or_insert(0);
                    *e += 1;
                } else {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
    }

    surface_cubes.values().sum()
}

impl Day for Day18 {
    fn star1(&self, input: &str) -> String {
        let cubes: Vec<_> = parse_input(input).collect();
        format!("{}", surface_area(&cubes))
    }

    fn star2(&self, input: &str) -> String {
        let cubes: Vec<_> = parse_input(input).collect();
        format!("{}", flood_outside_area(&cubes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;

        let d = Day18 {};
        assert_eq!(d.star1(input), "64");
        assert_eq!(d.star2(input), "58");
    }
}
