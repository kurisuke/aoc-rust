use crate::day::Day;
use crate::grid2d::Grid2D;
use itertools::iproduct;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Add;

pub struct Day17 {}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Coords3D(i64, i64, i64);

impl Add for Coords3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coords3D(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
struct Coords4D(i64, i64, i64, i64);

impl Add for Coords4D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Coords4D(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

fn parse_input_3d(input: &str) -> HashSet<Coords3D> {
    let init_grid = Grid2D::new(input).unwrap();
    init_grid
        .enumerate()
        .filter(|pos| *pos.1 == '#')
        .map(|pos| Coords3D(pos.0.x, pos.0.y, 0))
        .collect()
}

fn parse_input_4d(input: &str) -> HashSet<Coords4D> {
    let init_grid = Grid2D::new(input).unwrap();
    init_grid
        .enumerate()
        .filter(|pos| *pos.1 == '#')
        .map(|pos| Coords4D(pos.0.x, pos.0.y, 0, 0))
        .collect()
}

fn active_neighbors<T>(cube: &HashSet<T>, pos: T, neighbors: &[T], zero_el: &T) -> usize
where
    T: std::ops::Add<Output = T> + Eq + Hash + Copy,
{
    neighbors
        .iter()
        .filter(|diff| diff != &zero_el && cube.contains(&(pos + **diff)))
        .count()
}

fn iterate<T>(mut cube: HashSet<T>, n: usize, neighbors: &[T], zero_el: &T) -> usize
where
    T: std::ops::Add<Output = T> + Eq + Hash + Copy,
{
    for _ in 0..n {
        let mut new_cube: HashSet<T> = HashSet::new();
        let mut checked_pos: HashSet<T> = HashSet::new();
        for pos in cube.iter() {
            for diff in neighbors.iter() {
                let new_coords = *pos + *diff;
                if !checked_pos.contains(&new_coords) {
                    let is_active = match active_neighbors(&cube, new_coords, &neighbors, &zero_el)
                    {
                        0..=1 => false,
                        2 => cube.contains(&new_coords),
                        3 => true,
                        _ => false,
                    };
                    if is_active {
                        new_cube.insert(new_coords);
                    }
                    checked_pos.insert(new_coords);
                }
            }
        }
        cube = new_cube;
    }
    cube.len()
}

impl Day for Day17 {
    fn star1(&self, input: &str) -> String {
        let cube = parse_input_3d(input);
        let neighbors: Vec<_> = iproduct!(-1..=1, -1..=1, -1..=1)
            .map(|c| Coords3D(c.0, c.1, c.2))
            .collect();
        let zero_el = Coords3D(0, 0, 0);
        format!("{}", iterate(cube, 6, &neighbors, &zero_el))
    }

    fn star2(&self, input: &str) -> String {
        let hypercube = parse_input_4d(input);
        let neighbors: Vec<_> = iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
            .map(|c| Coords4D(c.0, c.1, c.2, c.3))
            .collect();
        let zero_el = Coords4D(0, 0, 0, 0);
        format!("{}", iterate(hypercube, 6, &neighbors, &zero_el))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#".#.
..#
###"#;
        let d = Day17 {};
        assert_eq!(d.star1(input), "112");
        assert_eq!(d.star2(input), "848");
    }
}
