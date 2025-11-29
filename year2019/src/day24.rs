use common::day::Day;
use std::collections::{HashMap, HashSet};
use util::grid2d::{Coords, Grid2D};

pub struct Day24 {}

type Grids = HashMap<i64, Grid2D<char>>;

fn rating(grid: &Grid2D<char>) -> usize {
    let mut rating = 0;
    for (i, c) in grid.coords_iter().enumerate() {
        rating += (1 << i) * usize::from(grid.at(&c).unwrap() == &'#');
    }
    rating
}

fn evolve(grid: &Grid2D<char>) -> Grid2D<char> {
    let mut new_grid = Grid2D::with_default(Coords { x: 5, y: 5 }, &'.');
    for c in grid.coords_iter() {
        let cnt = grid
            .neighbors_cardinal(&c)
            .into_iter()
            .filter(|n| n == &Some(&'#'))
            .count();
        match grid.at(&c).unwrap() {
            '#' => {
                if cnt == 1 {
                    new_grid.set(&c, '#');
                }
            }
            '.' => {
                if cnt == 1 || cnt == 2 {
                    new_grid.set(&c, '#');
                }
            }
            _ => {
                unreachable!();
            }
        }
    }
    new_grid
}

fn evolve_rec(grids: &Grids) -> Grids {
    let iter = if grids.len().is_multiple_of(2) {
        grids.len() / 2
    } else {
        grids.len() / 2 + 1
    } as i64;
    let mut new_grids = HashMap::new();
    let grid_def = Grid2D::with_default(Coords { x: 5, y: 5 }, &'.');

    for i in -iter..=iter {
        let grid = grids.get(&i).unwrap_or(&grid_def);
        let grid_below = grids.get(&(i - 1)).unwrap_or(&grid_def);
        let grid_above = grids.get(&(i + 1)).unwrap_or(&grid_def);

        let new_grid = evolve2(grid, grid_below, grid_above);
        new_grids.insert(i, new_grid);
    }
    new_grids
}

fn evolve2(
    grid: &Grid2D<char>,
    grid_above: &Grid2D<char>,
    grid_below: &Grid2D<char>,
) -> Grid2D<char> {
    let mut new_grid = Grid2D::with_default(Coords { x: 5, y: 5 }, &'.');
    for c in grid.coords_iter() {
        if c.y == 2 && c.x == 2 {
            new_grid.set(&c, '?');
            continue;
        }

        let cnt = count_neighbors2(grid, grid_above, grid_below, &c);
        match grid.at(&c).unwrap() {
            '#' => {
                if cnt == 1 {
                    new_grid.set(&c, '#');
                }
            }
            '.' => {
                if cnt == 1 || cnt == 2 {
                    new_grid.set(&c, '#');
                }
            }
            _ => {
                unreachable!();
            }
        }
    }
    new_grid
}

fn count_neighbors2(
    grid: &Grid2D<char>,
    grid_above: &Grid2D<char>,
    grid_below: &Grid2D<char>,
    c: &Coords,
) -> usize {
    let mut neighbors = vec![];

    for n in grid.neighbors_cardinal_coords(c) {
        // from grid above
        if n.y == -1 {
            neighbors.push(grid_above.at(&Coords { y: 1, x: 2 }).unwrap());
        } else if n.y == 5 {
            neighbors.push(grid_above.at(&Coords { y: 3, x: 2 }).unwrap());
        }

        if n.x == -1 {
            neighbors.push(grid_above.at(&Coords { y: 2, x: 1 }).unwrap());
        } else if n.x == 5 {
            neighbors.push(grid_above.at(&Coords { y: 2, x: 3 }).unwrap());
        }

        // from grid below
        if c.y == 1 && c.x == 2 && n.y == 2 && n.x == 2 {
            neighbors.push(grid_below.at(&Coords { y: 0, x: 0 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 0, x: 1 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 0, x: 2 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 0, x: 3 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 0, x: 4 }).unwrap());
            continue;
        }

        if c.y == 3 && c.x == 2 && n.y == 2 && n.x == 2 {
            neighbors.push(grid_below.at(&Coords { y: 4, x: 0 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 4, x: 1 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 4, x: 2 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 4, x: 3 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 4, x: 4 }).unwrap());
            continue;
        }

        if c.y == 2 && c.x == 1 && n.y == 2 && n.x == 2 {
            neighbors.push(grid_below.at(&Coords { y: 0, x: 0 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 1, x: 0 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 2, x: 0 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 3, x: 0 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 4, x: 0 }).unwrap());
            continue;
        }

        if c.y == 2 && c.x == 3 && n.y == 2 && n.x == 2 {
            neighbors.push(grid_below.at(&Coords { y: 0, x: 4 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 1, x: 4 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 2, x: 4 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 3, x: 4 }).unwrap());
            neighbors.push(grid_below.at(&Coords { y: 4, x: 4 }).unwrap());
            continue;
        }

        if let Some(v) = grid.at(&n) {
            neighbors.push(v);
        }
    }
    neighbors.into_iter().filter(|n| n == &&'#').count()
}

fn evolve_rec_n(input: &str, n: usize) -> Grids {
    let mut grids = HashMap::new();
    grids.insert(0, Grid2D::new(input).unwrap());
    for _ in 0..n {
        grids = evolve_rec(&grids);
    }
    grids
}

impl Day for Day24 {
    fn star1(&self, input: &str) -> String {
        let mut grid = Grid2D::new(input).unwrap();
        let mut ratings = HashSet::new();
        loop {
            let rating = rating(&grid);
            if !ratings.insert(rating) {
                return format!("{}", rating);
            }
            grid = evolve(&grid);
        }
    }

    fn star2(&self, input: &str) -> String {
        let grids = evolve_rec_n(input, 200);
        format!(
            "{}",
            grids.values().map(|grid| grid.count('#')).sum::<usize>()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"....#
#..#.
#..##
..#..
#...."#;
        let d = Day24 {};
        assert_eq!(d.star1(input), "2129920");

        let grids = evolve_rec_n(input, 10);
        assert_eq!(
            grids.values().map(|grid| grid.count('#')).sum::<usize>(),
            99
        );
    }
}
