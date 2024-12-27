use std::collections::HashSet;

use common::day::Day;
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day06 {}

impl Day for Day06 {
    fn star1(&self, input: &str) -> String {
        let mut grid = Grid2D::new(input).unwrap();
        let start_pos = grid.find('^').unwrap();
        grid.set(&start_pos, '.');

        walk(&grid, start_pos, None, true)
            .unwrap()
            .unwrap()
            .len()
            .to_string()
    }

    fn star2(&self, input: &str) -> String {
        let mut grid = Grid2D::new(input).unwrap();
        let start_pos = grid.find('^').unwrap();
        grid.set(&start_pos, '.');

        // when adding an obstacle, only consider positions the guard would actually visit
        // on their normal walk, but ignore their start position
        let mut visited_pos = walk(&grid, start_pos, None, true).unwrap().unwrap();
        visited_pos.remove(&start_pos);

        let mut num_loop_pos = 0;
        for add_pos in visited_pos {
            if walk(&grid, start_pos, Some(add_pos), false).is_err() {
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

impl Guard {
    fn id(&self) -> usize {
        self.pos.x as usize
            + ((self.pos.y as usize) << 8)
            + (match self.dir {
                Direction::N => 0,
                Direction::E => 1,
                Direction::S => 2,
                Direction::W => 3,
                _ => unreachable!(),
            } << 16)
    }
}

fn walk(
    grid: &Grid2D<char>,
    pos: Coords,
    obstacle: Option<Coords>,
    count_pos: bool,
) -> Result<Option<HashSet<Coords>>, ()> {
    let mut guard = Guard {
        pos,
        dir: Direction::N,
    };

    let mut visited = vec![false; 1 << 18];
    visited[guard.id()] = true;
    let mut positions = HashSet::new();
    if count_pos {
        positions.insert(guard.pos);
    }

    loop {
        let check_pos = guard.pos.mov(guard.dir);
        if let Some(v) = grid.at(&check_pos) {
            if obstacle.is_some() && check_pos == obstacle.unwrap() {
                guard.dir = match guard.dir {
                    Direction::N => Direction::E,
                    Direction::E => Direction::S,
                    Direction::S => Direction::W,
                    Direction::W => Direction::N,
                    _ => unreachable!(),
                };
                continue;
            }

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
                    if count_pos {
                        positions.insert(guard.pos);
                    }

                    if visited[guard.id()] {
                        return Err(());
                    } else {
                        visited[guard.id()] = true;
                    }
                }
                _ => unreachable!(),
            }
        } else {
            break;
        }
    }

    if count_pos {
        Ok(Some(positions))
    } else {
        Ok(None)
    }
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
