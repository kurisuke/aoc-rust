use crate::day::Day;
use crate::util::grid2d::{Coords, Grid2D};

pub struct Day18 {}

fn step(grid: &Grid2D<char>) -> Grid2D<char> {
    let mut new_grid = grid.clone();
    for (pos, value) in grid.enumerate() {
        let active_neighbors = grid
            .neighbors(&pos)
            .into_iter()
            .filter_map(|x| x)
            .filter(|x| **x == '#')
            .count();
        if active_neighbors == 3 || (active_neighbors == 2 && *value == '#') {
            new_grid.set(&pos, '#');
        } else {
            new_grid.set(&pos, '.');
        }
    }
    new_grid
}

fn turn_on_corners(grid: &mut Grid2D<char>) {
    grid.set(&Coords { x: 0, y: 0 }, '#');
    grid.set(
        &Coords {
            x: 0,
            y: grid.height() - 1,
        },
        '#',
    );
    grid.set(
        &Coords {
            x: grid.width() - 1,
            y: 0,
        },
        '#',
    );
    grid.set(
        &Coords {
            x: grid.width() - 1,
            y: grid.height() - 1,
        },
        '#',
    );
}

impl Day for Day18 {
    fn star1(&self, input: &str) -> String {
        let mut grid = Grid2D::new(input).unwrap();
        for _ in 0..100 {
            grid = step(&grid);
        }
        format!("{}", grid.count('#'))
    }

    fn star2(&self, input: &str) -> String {
        let mut grid = Grid2D::new(input).unwrap();
        for _ in 0..100 {
            turn_on_corners(&mut grid);
            grid = step(&grid);
        }
        turn_on_corners(&mut grid);
        format!("{}", grid.count('#'))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn star1() {
        let input = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#;
        let mut grid = Grid2D::new(input).unwrap();
        for _ in 0..4 {
            grid = step(&grid);
        }
        assert_eq!(grid.count('#'), 4);
    }

    #[test]
    fn star2() {
        let input = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#;
        let mut grid = Grid2D::new(input).unwrap();
        for _ in 0..5 {
            turn_on_corners(&mut grid);
            grid = step(&grid);
        }
        turn_on_corners(&mut grid);
        assert_eq!(grid.count('#'), 17);
    }
}
