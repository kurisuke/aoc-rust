use std::collections::HashSet;

use common::day::Day;
use util::grid2d::{Coords, Grid2D};

pub struct Day21 {}

impl Day for Day21 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        search(&grid, 64, false).to_string()
    }

    fn star2(&self, input: &str) -> String {
        let target_steps = 26501365;
        let grid = Grid2D::new(input).unwrap();
        assert_eq!(grid.width(), grid.height());

        let offset = target_steps % grid.height();
        let x = (target_steps - offset) / grid.height();

        let xns: Vec<_> = (0..3)
            .map(|i| {
                let x = offset + i * grid.height();
                search(&grid, x as usize, true) as i64
            })
            .collect();

        let c = xns[0];
        let a = (xns[2] - 2 * xns[1] + c) / 2;
        let b = xns[1] - a - c;

        let y = a * x * x + b * x + c;
        y.to_string()
    }
}

fn search(grid: &Grid2D<char>, target_steps: usize, wrap: bool) -> usize {
    let start = grid.find('S').unwrap();

    let mut frontier = HashSet::new();
    let mut frontier_new = HashSet::new();
    frontier_new.insert(start);

    for _ in 0..target_steps {
        frontier.clear();
        frontier.extend(frontier_new.drain());
        for pos in &frontier {
            for neighbor in grid.neighbors_cardinal_coords(pos) {
                let check_coord = if wrap {
                    Coords {
                        x: neighbor.x.rem_euclid(grid.width()),
                        y: neighbor.y.rem_euclid(grid.height()),
                    }
                } else {
                    neighbor
                };

                if !frontier.contains(&neighbor)
                    && grid
                        .at(&check_coord)
                        .is_some_and(|c| c == &'.' || c == &'S')
                {
                    frontier_new.insert(neighbor);
                }
            }
        }
    }

    frontier_new.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

    #[test]
    fn ex1() {
        let grid = Grid2D::new(INPUT).unwrap();
        assert_eq!(search(&grid, 6, false), 16);
    }

    #[test]
    fn ex2() {
        let grid = Grid2D::new(INPUT).unwrap();
        assert_eq!(search(&grid, 100, true), 6536);
    }
}
