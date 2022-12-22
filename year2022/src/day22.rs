use common::day::Day;
use util::grid2d::{Coords, Direction, Grid2D};

pub struct Day22 {}

type SideMap = [[(usize, Face, bool); 4]; 6];

const TILES_PT1: SideMap = [
    [
        (1, Face::E, false),
        (2, Face::S, false),
        (1, Face::W, false),
        (4, Face::N, false),
    ],
    [
        (0, Face::E, false),
        (1, Face::S, false),
        (0, Face::W, false),
        (1, Face::N, false),
    ],
    [
        (2, Face::E, false),
        (4, Face::S, false),
        (2, Face::W, false),
        (0, Face::N, false),
    ],
    [
        (4, Face::E, false),
        (5, Face::S, false),
        (4, Face::W, false),
        (5, Face::N, false),
    ],
    [
        (3, Face::E, false),
        (0, Face::S, false),
        (3, Face::W, false),
        (2, Face::N, false),
    ],
    [
        (5, Face::E, false),
        (3, Face::S, false),
        (5, Face::W, false),
        (3, Face::N, false),
    ],
];

const TILES_PT2: SideMap = [
    [
        (1, Face::E, false),
        (2, Face::S, false),
        (3, Face::E, true),
        (5, Face::E, false),
    ],
    [
        (4, Face::W, true),
        (2, Face::W, false),
        (0, Face::W, false),
        (5, Face::N, false),
    ],
    [
        (1, Face::N, false),
        (4, Face::S, false),
        (3, Face::S, false),
        (0, Face::N, false),
    ],
    [
        (4, Face::E, false),
        (5, Face::S, false),
        (0, Face::E, true),
        (2, Face::E, false),
    ],
    [
        (1, Face::W, true),
        (5, Face::W, false),
        (3, Face::W, false),
        (2, Face::N, false),
    ],
    [
        (4, Face::N, false),
        (1, Face::S, false),
        (0, Face::S, false),
        (3, Face::N, false),
    ],
];

struct Side {
    top_left: Coords,
    tiles: Grid2D<char>,
}

fn parse_sides(input: &str, length: i64) -> Vec<Side> {
    let lines: Vec<_> = input.lines().collect();

    let mut sides = vec![];
    for (row_idx, row) in lines.chunks(length as usize).enumerate() {
        let row = row.join("\n");
        let g = Grid2D::new(&row).unwrap();
        let cols = g.width() / length;

        for col in 0..cols {
            let top_left = Coords {
                x: col * length,
                y: 0,
            };
            let bottom_right = Coords {
                x: (col + 1) * length,
                y: length,
            };
            let tiles = g.clip(top_left, bottom_right).unwrap();
            if tiles.at(&Coords { x: 0, y: 0 }).unwrap() != &' ' {
                sides.push(Side {
                    top_left: top_left
                        + Coords {
                            x: 0,
                            y: row_idx as i64 * length,
                        },
                    tiles,
                });
            }
        }
    }
    sides
}

#[derive(Debug)]
struct Position {
    side: usize,
    coords: Coords,
    face: Face,
}

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
enum Face {
    E = 0,
    S = 1,
    W = 2,
    N = 3,
}

impl Position {
    fn init() -> Position {
        Position {
            side: 0,
            coords: Coords { x: 0, y: 0 },
            face: Face::E,
        }
    }

    fn password(&self, sides: &[Side]) -> usize {
        let coords_global = sides[self.side].top_left + self.coords;
        (coords_global.y + 1) as usize * 1000
            + (coords_global.x + 1) as usize * 4
            + self.face as usize
    }

    fn do_instruction(&mut self, sides: &[Side], side_map: &SideMap, instr: &Instruction) {
        match instr {
            Instruction::Move(steps) => {
                self.do_move(sides, side_map, *steps);
            }
            Instruction::TurnLeft => {
                self.face = match self.face {
                    Face::E => Face::N,
                    Face::S => Face::E,
                    Face::W => Face::S,
                    Face::N => Face::W,
                };
            }
            Instruction::TurnRight => {
                self.face = match self.face {
                    Face::E => Face::S,
                    Face::S => Face::W,
                    Face::W => Face::N,
                    Face::N => Face::E,
                };
            }
        }
    }

    fn do_move(&mut self, sides: &[Side], side_map: &SideMap, steps: usize) {
        let tile_length = sides[0].tiles.width();
        for _ in 0..steps {
            let mut next_coords = self.coords.mov(match self.face {
                Face::E => Direction::E,
                Face::S => Direction::S,
                Face::W => Direction::W,
                Face::N => Direction::N,
            });

            let over_edge = match self.face {
                Face::E => next_coords.x >= tile_length,
                Face::S => next_coords.y >= tile_length,
                Face::W => next_coords.x < 0,
                Face::N => next_coords.y < 0,
            };

            if over_edge {
                let coord_in_bound = match self.face {
                    Face::E | Face::W => self.coords.y,
                    Face::N | Face::S => self.coords.x,
                };
                let (side_next, face_next, flip) = side_map[self.side][self.face as usize];
                next_coords = match (face_next, flip) {
                    (Face::E, true) => Coords {
                        x: 0,
                        y: tile_length - 1 - coord_in_bound,
                    },
                    (Face::E, false) => Coords {
                        x: 0,
                        y: coord_in_bound,
                    },
                    (Face::S, true) => Coords {
                        x: tile_length - 1 - coord_in_bound,
                        y: 0,
                    },
                    (Face::S, false) => Coords {
                        x: coord_in_bound,
                        y: 0,
                    },
                    (Face::W, true) => Coords {
                        x: tile_length - 1,
                        y: tile_length - 1 - coord_in_bound,
                    },
                    (Face::W, false) => Coords {
                        x: tile_length - 1,
                        y: coord_in_bound,
                    },
                    (Face::N, true) => Coords {
                        x: tile_length - 1 - coord_in_bound,
                        y: tile_length - 1,
                    },
                    (Face::N, false) => Coords {
                        x: coord_in_bound,
                        y: tile_length - 1,
                    },
                };
                if sides[side_next].tiles.at(&next_coords).unwrap() == &'#' {
                    break;
                } else {
                    self.side = side_next;
                    self.coords = next_coords;
                    self.face = face_next;
                }
            } else if sides[self.side].tiles.at(&next_coords).unwrap() == &'#' {
                break;
            } else {
                self.coords = next_coords;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let mut num_string = String::new();
    let mut instructions = vec![];
    for c in line.trim().chars() {
        if c.is_ascii_digit() {
            num_string.push(c);
        } else {
            if !num_string.is_empty() {
                instructions.push(Instruction::Move(num_string.parse().unwrap()));
                num_string.clear();
            }
            instructions.push(match c {
                'L' => Instruction::TurnLeft,
                'R' => Instruction::TurnRight,
                _ => unreachable!(),
            });
        }
    }
    if !num_string.is_empty() {
        instructions.push(Instruction::Move(num_string.parse().unwrap()));
    }
    instructions
}

fn run(input: &str, length: i64, side_map: &SideMap) -> usize {
    let mut input_it = input.split("\n\n");
    let input_map = input_it.next().unwrap();
    let input_instructions = input_it.next().unwrap();

    let sides = parse_sides(input_map, length);
    let instrs = parse_instructions(input_instructions);

    let mut pos = Position::init();
    for instr in instrs.iter() {
        pos.do_instruction(&sides, side_map, instr);
    }

    pos.password(&sides)
}

impl Day for Day22 {
    fn star1(&self, input: &str) -> String {
        format!("{}", run(input, 50, &TILES_PT1))
    }

    fn star2(&self, input: &str) -> String {
        format!("{}", run(input, 50, &TILES_PT2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TILES_PT1_TEST: SideMap = [
        [
            (0, Face::E, false),
            (3, Face::S, false),
            (0, Face::W, false),
            (4, Face::N, false),
        ],
        [
            (2, Face::E, false),
            (1, Face::S, false),
            (3, Face::W, false),
            (1, Face::N, false),
        ],
        [
            (3, Face::E, false),
            (2, Face::S, false),
            (1, Face::W, false),
            (2, Face::N, false),
        ],
        [
            (1, Face::E, false),
            (4, Face::S, false),
            (2, Face::W, false),
            (0, Face::N, false),
        ],
        [
            (5, Face::E, false),
            (0, Face::S, false),
            (5, Face::W, false),
            (3, Face::N, false),
        ],
        [
            (4, Face::E, false),
            (5, Face::S, false),
            (4, Face::W, false),
            (5, Face::N, false),
        ],
    ];

    const TILES_PT2_TEST: SideMap = [
        [
            (5, Face::W, true),
            (3, Face::S, false),
            (2, Face::S, false),
            (1, Face::S, true),
        ],
        [
            (2, Face::E, false),
            (4, Face::N, true),
            (5, Face::N, true),
            (0, Face::S, true),
        ],
        [
            (3, Face::E, false),
            (4, Face::E, true),
            (1, Face::W, false),
            (0, Face::E, false),
        ],
        [
            (5, Face::S, true),
            (4, Face::S, false),
            (2, Face::W, false),
            (0, Face::N, false),
        ],
        [
            (5, Face::E, false),
            (1, Face::N, true),
            (2, Face::N, true),
            (3, Face::N, false),
        ],
        [
            (0, Face::W, true),
            (1, Face::E, true),
            (4, Face::W, false),
            (3, Face::W, true),
        ],
    ];

    #[test]
    fn ex1() {
        let input = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;

        assert_eq!(run(input, 4, &TILES_PT1_TEST), 6032);
        assert_eq!(run(input, 4, &TILES_PT2_TEST), 5031);
    }
}
