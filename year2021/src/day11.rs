use common::day::Day;

use std::collections::HashSet;
use util::grid2d::{Coords, Grid2D};

pub struct Day11 {}

fn parse_input(input: &str) -> Grid2D<u32> {
    Grid2D::new_by(input, |c| c.to_digit(10).unwrap()).unwrap()
}

fn step(grid: &mut Grid2D<u32>) -> usize {
    // increment 1 for all
    let cs: Vec<_> = grid.coords_iter().collect();
    for c in cs.iter() {
        let v = *grid.at(c).unwrap();
        grid.set(c, v + 1);
    }

    // increase energy level for flash neighbors
    let mut has_flashed = HashSet::new();
    while let Some(flash_coord) = cs
        .iter()
        .find(|c| !has_flashed.contains(c) && *grid.at(c).unwrap() > 9)
    {
        has_flashed.insert(flash_coord);
        let ns: Vec<Coords> = grid.neighbors_coords(flash_coord).into_iter().collect();
        for n in ns {
            if let Some(&v) = grid.at(&n) {
                grid.set(&n, v + 1);
            }
        }
    }

    // reset all flashed to 0
    for c in has_flashed.iter() {
        grid.set(c, 0);
    }

    has_flashed.len()
}

impl Day for Day11 {
    fn star1(&self, input: &str) -> String {
        let mut grid = parse_input(input);
        let mut total_flashes = 0;
        for _ in 0..100 {
            total_flashes += step(&mut grid);
        }
        format!("{}", total_flashes)
    }

    fn star2(&self, input: &str) -> String {
        let mut grid = parse_input(input);
        let mut steps = 0;
        let num_octopuses = (grid.width() * grid.height()) as usize;
        loop {
            steps += 1;
            let flashes = step(&mut grid);
            if flashes == num_octopuses {
                break;
            }
        }
        format!("{}", steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

        let d = Day11 {};
        assert_eq!(d.star1(input), "1656");
        assert_eq!(d.star2(input), "195");
    }
}
