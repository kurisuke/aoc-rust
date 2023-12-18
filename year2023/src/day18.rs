use std::collections::{HashSet, VecDeque};

use common::day::Day;
use util::grid2d::{Coords, Direction};

pub struct Day18 {}

impl Day for Day18 {
    fn star1(&self, input: &str) -> String {
        let instrs: Vec<_> = input.lines().map(Instruction::parse).collect();
        let outline = outline(&instrs);
        area(&outline).to_string()
    }

    fn star2(&self, _input: &str) -> String {
        String::from("not implemented")
    }
}

struct Instruction {
    dir: Direction,
    length: i64,
    length_pt2: u32,
}

impl Instruction {
    fn parse(line: &str) -> Instruction {
        let secs: Vec<_> = line.split_whitespace().collect();
        let dir = match secs[0].chars().next().unwrap() {
            'U' => Direction::N,
            'D' => Direction::S,
            'L' => Direction::W,
            'R' => Direction::E,
            _ => unreachable!(),
        };
        let length = secs[1].parse().unwrap();
        let color_rgb = u32::from_str_radix(&secs[2][2..8], 16).unwrap();
        Instruction {
            dir,
            length,
            length_pt2: color_rgb,
        }
    }
}

fn outline(instrs: &[Instruction]) -> HashSet<Coords> {
    let mut ret = HashSet::new();
    let pos_start = Coords { x: 0, y: 0 };
    let mut pos = pos_start;
    ret.insert(pos);
    for instr in instrs {
        for _ in 0..instr.length {
            pos = pos.mov(instr.dir);
            ret.insert(pos);
        }
    }
    assert_eq!(pos, pos_start);
    ret
}

fn area(outline_coords: &HashSet<Coords>) -> i64 {
    let (x_min, x_max, y_min, y_max) = outline_coords.iter().fold(
        (i64::MAX, i64::MIN, i64::MAX, i64::MIN),
        |(x_min, x_max, y_min, y_max), c| {
            (
                x_min.min(c.x),
                x_max.max(c.x),
                y_min.min(c.y),
                y_max.max(c.y),
            )
        },
    );

    // one more row / column so we have a connected outside
    let (x_min, x_max, y_min, y_max) = (x_min - 1, x_max + 1, y_min - 1, y_max + 1);
    let total_area = (x_max - x_min + 1) * (y_max - y_min + 1);

    println!("total area: {total_area}");

    // flood fill
    let mut outside = HashSet::new();
    let mut queue = VecDeque::new();
    let start_pos = Coords { x: x_min, y: y_min };
    queue.push_back(start_pos);
    outside.insert(start_pos);

    while let Some(pos) = queue.pop_front() {
        for d in [Direction::N, Direction::E, Direction::S, Direction::W] {
            let new_pos = pos.mov(d);
            if new_pos.x >= x_min
                && new_pos.x <= x_max
                && new_pos.y >= y_min
                && new_pos.y <= y_max
                && !outline_coords.contains(&new_pos)
                && !outside.contains(&new_pos)
            {
                outside.insert(new_pos);
                queue.push_back(new_pos);
            }
        }
    }

    total_area - outside.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn ex1() {
        let d = Day18 {};
        assert_eq!(d.star1(INPUT), "62");
    }

    #[test]
    fn test_parse() {
        let instr = Instruction::parse("R 6 (#70c710)");
        assert_eq!(instr.dir, Direction::E);
        assert_eq!(instr.length, 6);
        assert_eq!(instr.length_pt2, 0x70c710);
    }
}
