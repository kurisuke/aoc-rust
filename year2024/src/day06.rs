use std::collections::HashSet;

use common::day::Day;
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day06 {}

impl Day for Day06 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        let visited = walk(grid).unwrap();
        let visited: HashSet<_> = visited.into_iter().map(|g| g.pos).collect();
        format!("{}", visited.len())
    }

    fn star2(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();

        // when adding an obstacle, only consider positions the guard would actually visit
        // on their normal walk, but ignore their start position
        let visited = walk(grid.clone()).unwrap();
        let mut visited_pos: HashSet<_> = visited.into_iter().map(|g| g.pos).collect();
        visited_pos.remove(&grid.find('^').unwrap());

        let mut num_loop_pos = 0;
        for add_pos in visited_pos {
            let mut grid_add = grid.clone();
            grid_add.set(&add_pos, '#');
            if walk(grid_add).is_err() {
                num_loop_pos += 1;
            }
        }
        format!("{}", num_loop_pos)
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct Guard {
    pos: Coords,
    dir: Direction,
}

fn walk(mut grid: Grid2D<char>) -> Result<HashSet<Guard>, Guard> {
    let pos = grid.find('^').unwrap();
    let mut guard = Guard {
        pos,
        dir: Direction::N,
    };

    let mut visited = HashSet::new();
    visited.insert(guard);
    grid.set(&pos, '.');

    loop {
        let check_pos = guard.pos.mov(guard.dir);
        if let Some(v) = grid.at(&check_pos) {
            match v {
                '#' => {
                    guard.dir = match guard.dir {
                        Direction::N => Direction::E,
                        Direction::E => Direction::S,
                        Direction::S => Direction::W,
                        Direction::W => Direction::N,
                        _ => unreachable!(),
                    };
                }
                '.' => {
                    guard.pos = check_pos;
                    if !visited.insert(guard) {
                        return Err(guard);
                    }
                }
                _ => unreachable!(),
            }
        } else {
            break;
        }
    }

    Ok(visited)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn star1() {
        let d = Day06 {};
        assert_eq!(d.star1(INPUT), "41");
    }

    #[test]
    fn star2() {
        let d = Day06 {};
        assert_eq!(d.star2(INPUT), "6");
    }
}
