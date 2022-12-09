use common::day::Day;
use std::collections::HashSet;
use std::ops::{Add, Sub};

pub struct Day09 {}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Pos(i64, i64);

const DIRS: [Pos; 8] = [
    Pos(-1, -1),
    Pos(0, -1),
    Pos(1, -1),
    Pos(-1, 0),
    Pos(1, 0),
    Pos(-1, 1),
    Pos(0, 1),
    Pos(1, 1),
];

impl Pos {
    fn moves(&self, other: &Pos) -> u64 {
        (self.0 - other.0)
            .unsigned_abs()
            .max((self.1 - other.1).unsigned_abs())
    }

    fn manhattan(&self, other: &Pos) -> u64 {
        (self.0 - other.0).unsigned_abs() + (self.1 - other.1).unsigned_abs()
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

fn move_tail(head: &Pos, tail: &Pos) -> Pos {
    let delta_moves = head.moves(tail);
    if delta_moves > 1 {
        DIRS.iter()
            .map(|d| *tail + *d)
            .min_by(|t1, t2| head.manhattan(t1).cmp(&head.manhattan(t2)))
            .unwrap()
    } else {
        *tail
    }
}

struct MoveCmd {
    dir: Pos,
    repeat: usize,
}

fn parse_input(input: &str) -> impl Iterator<Item = MoveCmd> + '_ {
    input.lines().map(|line| {
        let mut tokens = line.split_whitespace();
        let dir = match tokens.next().unwrap() {
            "L" => Pos(-1, 0),
            "R" => Pos(1, 0),
            "U" => Pos(0, -1),
            "D" => Pos(0, 1),
            _ => unreachable!(),
        };
        let repeat = tokens.next().unwrap().parse().unwrap();
        MoveCmd { dir, repeat }
    })
}

impl Day for Day09 {
    fn star1(&self, input: &str) -> String {
        let cmds = parse_input(input);
        let mut head = Pos(0, 0);
        let mut tail = Pos(0, 0);
        let mut tail_pos = HashSet::new();

        for cmd in cmds {
            for _ in 0..cmd.repeat {
                head = head + cmd.dir;
                tail = move_tail(&head, &tail);
                tail_pos.insert(tail);
            }
        }

        format!("{}", tail_pos.len())
    }

    fn star2(&self, input: &str) -> String {
        let cmds = parse_input(input);
        let mut knots = [Pos(0, 0); 10];
        let mut tail_pos = HashSet::new();

        for cmd in cmds {
            for _ in 0..cmd.repeat {
                knots[0] = knots[0] + cmd.dir;
                for i in 1..=9 {
                    knots[i] = move_tail(&knots[i - 1], &knots[i]);
                }
                tail_pos.insert(knots[9]);
            }
        }

        format!("{}", tail_pos.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

        let d = Day09 {};
        assert_eq!(d.star1(input), "13");
        assert_eq!(d.star2(input), "1");
    }

    #[test]
    fn ex2() {
        let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

        let d = Day09 {};
        assert_eq!(d.star2(input), "36");
    }
}
