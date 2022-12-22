use common::day::Day;

pub struct Day22 {}

#[derive(Debug)]
struct Position {
    coords: (usize, usize),
    face: Face,
}

impl Position {
    fn init(map: &Map) -> Position {
        Position {
            coords: (0, map.horiz[0].start),
            face: Face::E,
        }
    }

    fn password(&self) -> usize {
        (self.coords.0 + 1) * 1000 + (self.coords.1 + 1) * 4 + self.face as usize
    }

    fn do_instruction(&mut self, map: &Map, instr: &Instruction) {
        match instr {
            Instruction::Move(mut steps) => {
                let (move_row, mut move_index) = match self.face {
                    Face::E | Face::W => (&map.horiz[self.coords.0], self.coords.1),
                    Face::N | Face::S => (&map.vert[self.coords.1], self.coords.0),
                };
                let move_offset = match self.face {
                    Face::S | Face::E => 1,
                    Face::N | Face::W => -1,
                };

                // zero based indexing of tiles vec
                move_index -= move_row.start;

                while steps > 0 {
                    let next_move_index = (move_index as isize + move_offset)
                        .rem_euclid(move_row.tiles.len() as isize)
                        as usize;
                    if move_row.tiles[next_move_index] {
                        // blocked
                        break;
                    } else {
                        move_index = next_move_index;
                        steps -= 1;
                    }
                }

                move_index += move_row.start;
                match self.face {
                    Face::E | Face::W => {
                        self.coords.1 = move_index;
                    }
                    Face::N | Face::S => {
                        self.coords.0 = move_index;
                    }
                }
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
}

#[repr(usize)]
#[derive(Copy, Clone, Debug)]
enum Face {
    E = 0,
    S = 1,
    W = 2,
    N = 3,
}

struct Map {
    horiz: Vec<MapRow>,
    vert: Vec<MapRow>,
}

#[derive(Debug)]
struct MapRow {
    start: usize,
    tiles: Vec<bool>,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Move(usize),
    TurnLeft,
    TurnRight,
}

fn parse_map(input: &str) -> Map {
    let lines: Vec<_> = input.lines().collect();

    let width = lines.iter().map(|l| l.len()).max().unwrap();

    // horiz
    let mut horiz = vec![];
    for l in lines.iter() {
        let start = l.chars().position(|c| c != ' ').unwrap();
        let tiles: Vec<_> = l[start..].chars().map(|c| c == '#').collect();
        let row = MapRow { start, tiles };
        // println!("{:?}", row);
        horiz.push(row);
    }

    // vert
    let mut vert = vec![];
    for col in 0..width {
        // find start
        let mut start = 0;
        while (horiz[start].start > col) || (horiz[start].start + horiz[start].tiles.len() <= col) {
            start += 1;
        }

        // find end
        let mut end = horiz.len() - 1;
        while (horiz[end].start > col) || (horiz[end].start + horiz[end].tiles.len() <= col) {
            end -= 1;
        }

        let mut tiles = vec![];
        for horiz_line in horiz.iter().take(end + 1).skip(start) {
            tiles.push(horiz_line.tiles[col - horiz_line.start]);
        }
        let row = MapRow { start, tiles };
        // println!("{}: {:?}", col, row);
        vert.push(row);
    }

    Map { horiz, vert }
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
    // println!("{:?}", instructions);
    instructions
}

impl Day for Day22 {
    fn star1(&self, input: &str) -> String {
        let mut input_it = input.split("\n\n");
        let input_map = input_it.next().unwrap();
        let input_instructions = input_it.next().unwrap();

        let map = parse_map(input_map);
        let instructions = parse_instructions(input_instructions);
        let mut pos = Position::init(&map);
        for instr in instructions.iter() {
            pos.do_instruction(&map, instr);
            // println!("{:?}", pos);
        }

        format!("{}", pos.password())
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let d = Day22 {};

        assert_eq!(d.star1(input), "6032");
    }
}
