use common::day::Day;
use util::grid2d::{Coords, Direction};

pub struct Day18 {}

impl Day for Day18 {
    fn star1(&self, input: &str) -> String {
        let instrs: Vec<_> = input.lines().map(Instruction::parse_pt1).collect();
        area(&instrs).to_string()
    }

    fn star2(&self, input: &str) -> String {
        let instrs: Vec<_> = input.lines().map(Instruction::parse_pt2).collect();
        area(&instrs).to_string()
    }
}

struct Instruction {
    dir: Direction,
    length: i64,
}

impl Instruction {
    fn parse_pt1(line: &str) -> Instruction {
        let secs: Vec<_> = line.split_whitespace().collect();
        let dir = match secs[0].chars().next().unwrap() {
            'U' => Direction::N,
            'D' => Direction::S,
            'L' => Direction::W,
            'R' => Direction::E,
            _ => unreachable!(),
        };
        let length = secs[1].parse().unwrap();
        Instruction { dir, length }
    }

    fn parse_pt2(line: &str) -> Instruction {
        let secs: Vec<_> = line.split_whitespace().collect();
        let length = i64::from_str_radix(&secs[2][2..7], 16).unwrap();
        let dir = match secs[2].chars().nth(7).unwrap() {
            '0' => Direction::E,
            '1' => Direction::S,
            '2' => Direction::W,
            '3' => Direction::N,
            _ => unreachable!(),
        };
        Instruction { dir, length }
    }
}

fn area(instrs: &[Instruction]) -> i64 {
    let b = instrs.iter().map(|instr| instr.length).sum::<i64>();
    let v = vertices(instrs);
    // Shoelace formula / Gauss trapezoid
    let i = area_shoelace(&v);
    // Pick's theorem
    let a = i - b / 2 + 1;
    // Apparently outline points are not counted in above formula!
    a + b
}

fn vertices(instrs: &[Instruction]) -> Vec<Coords> {
    let mut ret = vec![];
    let pos_start = Coords { x: 0, y: 0 };
    let mut pos = pos_start;
    ret.push(pos);
    for instr in instrs {
        match instr.dir {
            Direction::N => {
                pos.y -= instr.length;
            }
            Direction::E => {
                pos.x += instr.length;
            }
            Direction::S => {
                pos.y += instr.length;
            }
            Direction::W => {
                pos.x -= instr.length;
            }
            _ => unreachable!(),
        }
        ret.push(pos);
    }
    assert_eq!(pos, pos_start);
    ret
}

fn area_shoelace(vertices: &[Coords]) -> i64 {
    let mut a = 0;
    for i in 0..vertices.len() {
        let j = (i + 1) % vertices.len();
        a += (vertices[i].x * vertices[j].y) - (vertices[i].y * vertices[j].x);
    }
    a / 2
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
    fn ex2() {
        let d = Day18 {};
        assert_eq!(d.star2(INPUT), "952408144115");
    }

    #[test]
    fn test_parse_pt1() {
        let instr = Instruction::parse_pt1("R 6 (#70c710)");
        assert_eq!(instr.dir, Direction::E);
        assert_eq!(instr.length, 6);
    }

    #[test]
    fn test_parse_pt2() {
        let instr = Instruction::parse_pt2("R 6 (#70c710)");
        assert_eq!(instr.dir, Direction::E);
        assert_eq!(instr.length, 461937);
    }
}
