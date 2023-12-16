use std::collections::{HashSet, VecDeque};

use common::day::Day;
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day16 {}

impl Day for Day16 {
    fn star1(&self, input: &str) -> String {
        let grid = Grid2D::new(input).unwrap();
        energized(&grid).to_string()
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Beam {
    pos: Coords,
    dir: Direction,
}

fn energized(grid: &Grid2D<char>) -> usize {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();

    let init = Beam {
        pos: Coords { x: 0, y: 0 },
        dir: Direction::E,
    };
    seen.insert(init);
    queue.push_back(init);

    while let Some(beam) = queue.pop_front() {
        let beams_next = beam.next(&grid);
        for beam_next in beams_next.into_iter() {
            if grid.at(&beam_next.pos).is_some() && seen.insert(beam_next) {
                queue.push_back(beam_next);
            }
        }
    }

    let fields: HashSet<_> = seen.into_iter().map(|beam| beam.pos).collect();
    fields.len()
}

impl Beam {
    fn next(self, grid: &Grid2D<char>) -> Vec<Beam> {
        if let Some(c) = grid.at(&self.pos) {
            match c {
                '.' => {
                    let pos = self.pos.mov(self.dir);
                    vec![Beam { pos, dir: self.dir }]
                }
                '\\' => {
                    let dir = match self.dir {
                        Direction::N => Direction::W,
                        Direction::S => Direction::E,
                        Direction::E => Direction::S,
                        Direction::W => Direction::N,
                        _ => unreachable!(),
                    };
                    let pos = self.pos.mov(dir);
                    vec![Beam { pos, dir }]
                }
                '/' => {
                    let dir = match self.dir {
                        Direction::N => Direction::E,
                        Direction::S => Direction::W,
                        Direction::E => Direction::N,
                        Direction::W => Direction::S,
                        _ => unreachable!(),
                    };
                    let pos = self.pos.mov(dir);
                    vec![Beam { pos, dir }]
                }
                '|' => match self.dir {
                    Direction::E | Direction::W => {
                        vec![
                            Beam {
                                pos: self.pos.mov(Direction::N),
                                dir: Direction::N,
                            },
                            Beam {
                                pos: self.pos.mov(Direction::S),
                                dir: Direction::S,
                            },
                        ]
                    }
                    Direction::N | Direction::S => {
                        let pos = self.pos.mov(self.dir);
                        vec![Beam { pos, dir: self.dir }]
                    }
                    _ => unreachable!(),
                },
                '-' => match self.dir {
                    Direction::N | Direction::S => {
                        vec![
                            Beam {
                                pos: self.pos.mov(Direction::W),
                                dir: Direction::W,
                            },
                            Beam {
                                pos: self.pos.mov(Direction::E),
                                dir: Direction::E,
                            },
                        ]
                    }
                    Direction::E | Direction::W => {
                        let pos = self.pos.mov(self.dir);
                        vec![Beam { pos, dir: self.dir }]
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        } else {
            // outside grid
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn ex1() {
        let d = Day16 {};
        assert_eq!(d.star1(INPUT), "46");
    }
}
