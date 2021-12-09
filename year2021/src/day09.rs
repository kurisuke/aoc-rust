use common::day::Day;

use std::collections::HashSet;
use util::grid2d::{Coords, Grid2D};

pub struct Day09 {}

fn parse_input(input: &str) -> Grid2D<char> {
    Grid2D::new(input).unwrap()
}

fn find_low_points(grid: &Grid2D<char>) -> Vec<Coords> {
    let mut low_points = vec![];
    for c in grid.coords_iter() {
        let v = grid.at(&c).unwrap().to_digit(10).unwrap();
        if grid
            .neighbors_cardinal(&c)
            .iter()
            .filter_map(|n| *n)
            .all(|n| v < n.to_digit(10).unwrap())
        {
            low_points.push(c);
        }
    }
    low_points
}

fn risk_level(grid: &Grid2D<char>, low_points: &[Coords]) -> u32 {
    let mut sum = 0;
    for l in low_points {
        sum += 1 + grid.at(l).unwrap().to_digit(10).unwrap();
    }
    sum
}

fn basin_size(grid: &Grid2D<char>, low_point: &Coords) -> usize {
    let mut size = 0;
    let mut frontier = HashSet::new();
    frontier.insert(*low_point);
    let mut inside = HashSet::new();
    while !frontier.is_empty() {
        size += frontier.len();
        let mut new_frontier = HashSet::new();
        for f in frontier.iter() {
            for n in grid.neighbors_cardinal_coords(f) {
                if !new_frontier.contains(&n)
                    && !frontier.contains(&n)
                    && !inside.contains(&n)
                    && grid.at(&n).is_some()
                {
                    let n_val = grid.at(&n).unwrap().to_digit(10).unwrap();
                    if n_val < 9 {
                        new_frontier.insert(n);
                    }
                }
            }
        }
        inside = inside.union(&frontier).cloned().collect();
        frontier = new_frontier;
    }
    size
}

impl Day for Day09 {
    fn star1(&self, input: &str) -> String {
        let grid = parse_input(input);
        let low_points = find_low_points(&grid);
        let risk = risk_level(&grid, &low_points);
        format!("{}", risk)
    }

    fn star2(&self, input: &str) -> String {
        let grid = parse_input(input);
        let low_points = find_low_points(&grid);
        let mut basin_sizes: Vec<_> = low_points.iter().map(|l| basin_size(&grid, l)).collect();
        basin_sizes.sort_unstable();
        let largest: usize = basin_sizes.into_iter().rev().take(3).product();
        format!("{}", largest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

        let d = Day09 {};
        assert_eq!(d.star1(input), "15");
        assert_eq!(d.star2(input), "1134");
    }
}
