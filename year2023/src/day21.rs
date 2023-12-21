use std::collections::HashSet;

use common::day::Day;
use util::grid2d::Grid2D;

pub struct Day21 {}

impl Day for Day21 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        search(grid, 64).to_string()
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

fn search(grid: Grid2D<char>, target_steps: usize) -> usize {
    let start = grid.find('S').unwrap();

    let mut frontier = HashSet::new();
    let mut frontier_new = HashSet::new();
    frontier_new.insert(start);

    for _ in 0..target_steps {
        frontier.clear();
        frontier.extend(frontier_new.drain());
        for pos in &frontier {
            for neighbor in grid.neighbors_cardinal_coords(pos) {
                if !frontier.contains(&neighbor)
                    && grid.at(&neighbor).is_some_and(|c| c == &'.' || c == &'S')
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

    #[test]
    fn ex1() {
        let input = r#"...........
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

        let grid = Grid2D::new(input).unwrap();
        assert_eq!(search(grid, 6), 16)
    }
}
